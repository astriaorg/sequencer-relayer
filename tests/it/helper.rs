use k8s_openapi::api::{core::v1::Namespace, apps::v1::Deployment};
use kube::{api::{DynamicObject, PatchParams, Patch, PostParams},
discovery::{ApiCapabilities, ApiResource, Scope},
Client, Api, Discovery, core::{GroupVersionKind, ObjectMeta}, runtime::wait::{await_condition, conditions, Condition}, ResourceExt};
use once_cell::sync::Lazy;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

const TEST_ENVIRONMENT_YAML: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/test_environment/test-environment.yml"));

const TEST_INGRESS_TEMPLATE: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/test_environment/ingress.yml.j2"));

static STOP_POD_TX: Lazy<UnboundedSender<String>> = Lazy::new(|| {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let _ = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();
        rt.block_on(async move {
            let client = kube::Client::try_default()
                .await
                .expect("should be able to connect to kuberneter cluster; is it running?");
            while let Some(namespace) = rx.recv().await {
                let client = client.clone();
                // spawn "fire and forget" tasks so the force removes are sent
                // to podman immediately and without waiting for a server response.
                tokio::spawn(async move {
                    todo!("destroy namespace and deployment")
                });
            }
        });
    });
    tx
});

#[macro_export]
macro_rules! init_test {
    () => {{
        let test_environment = crate::helper::TestEnvironment::init().await;
        test_environment
    }}
}

pub struct TestEnvironment {
    host: String,
    namespace: String,
    tx: UnboundedSender<String>,
}

impl TestEnvironment {
    pub(crate) async fn init() -> Self {
        let namespace = Uuid::new_v4().simple().to_string();
        let client = Client::try_default()
            .await
            .expect("should be able to connect to kuberneter cluster; is it running?");
        let discovery = Discovery::new(client.clone())
            .run()
            .await
            .expect("should be able to run discovery against cluster");
        let documents = crate::helper::multidoc_deserialize(TEST_ENVIRONMENT_YAML)
            .expect("should have been able to deserialize valid kustomize generated yaml; rerun `just kustomize`?");

        // Create the unique namespace
        create_namespace(
            &namespace,
            client.clone(),
            &PostParams {
                dry_run: false,
                field_manager: Some("sequencer-relayer-test".to_string()),
            }
        ).await;

        // Apply the kustomize-generated kube yaml
        let ssapply = PatchParams::apply("sequencer-relayer-test").force();
        for doc in documents {
            apply_yaml_value(&namespace, client.clone(), doc, &ssapply, &discovery).await;
        }

        // Set up the ingress rule under the same namespace
        let mut jinja_env = minijinja::Environment::new();        
        jinja_env.add_template("ingress.yml", TEST_INGRESS_TEMPLATE).expect("compile-time loaded ingress should be valid jinja");
        let ingress_template = jinja_env.get_template("ingress.yml").expect("ingress.yml was just loaded, it should exist");
        let ingress_yaml = serde_yaml::from_str(
            &ingress_template
                .render(minijinja::context!(namespace => namespace))
                .expect("should be able to render the ingress jinja template")
        ).expect("should be able to parse rendered ingress yaml as serde_yaml Value");
        apply_yaml_value(&namespace, client.clone(), ingress_yaml, &ssapply, &discovery).await;

        // Wait for the deployment to become available; this usually takes much longer than
        // setting up ingress rules or anything else.
        let deployment_api: Api<Deployment> = Api::namespaced(client.clone(), &namespace);
        await_condition(
            deployment_api,
            "sequencer-relayer-environment-deployment",
            is_deployment_available(),
        ).await.unwrap();
        
        let host = format!("http://{namespace}.localdev.me");
        Self {
            host,
            namespace,
            tx: Lazy::force(&STOP_POD_TX).clone(),
        }
    }
}

fn is_deployment_available() -> impl Condition<Deployment> {
    move |obj: Option<&Deployment>| {
        if let Some(deployment) = &obj {
            if let Some(status) = &deployment.status {
                if let Some(conds) = &status.conditions {
                    if let Some(dcond) = conds.iter().find(|c| c.type_ == "Available") {
                        return dcond.status == "True";
                    }
                }
            }
        }
        false
    }
}

// impl Drop for TestEnvironment {
//     fn drop(&mut self) {
//         if let Err(e) = self.tx.send(self.pod_name.clone()) {
//             eprintln!(
//                 "failed sending pod `{name}` to cleanup task while dropping StackInfo: {e:?}",
//                 name = self.pod_name,
//             .)
//         }
//     }
// }


fn multidoc_deserialize(data: &str) -> eyre::Result<Vec<serde_yaml::Value>> {
    use serde::Deserialize;
    let mut docs = vec![];
    for de in serde_yaml::Deserializer::from_str(data) {
        docs.push(serde_yaml::Value::deserialize(de)?);
    }
    Ok(docs)
}

fn dynamic_api(ar: ApiResource, caps: ApiCapabilities, client: Client, namespace: &str) -> Api<DynamicObject> {
    if caps.scope == Scope::Cluster {
        Api::all_with(client, &ar)
    } else {
        Api::namespaced_with(client, namespace, &ar)
    }
}

async fn apply_yaml_value(namespace: &str, client: Client, document: serde_yaml::Value, ssapply: &PatchParams, discovery: &Discovery) {
    let obj: DynamicObject = serde_yaml::from_value(document)
        .expect("should have been able to read valid kustomize generated doc into dynamic object; rerun `just kustomize`?");
    let gvk = if let Some(tm) = &obj.types {
        GroupVersionKind::try_from(tm).expect("failed reading group version kind from dynamic object types")
    } else {
        panic!("cannot apply object without valid TypeMeta: {obj:?}");
    };
    let name = obj.name_any();
    let Some((ar, caps)) = discovery.resolve_gvk(&gvk) else {
        panic!("cannot apply document for unknown group version kind: {gvk:?}");
    };
    let api = dynamic_api(ar, caps, client, namespace);
    let data: serde_json::Value = serde_json::to_value(&obj)
        .expect("should have been able to turn DynamicObject serde_json Value");
    let _r = api
        .patch(&name, ssapply, &Patch::Apply(data))
        .await
        .expect("should have been able to apply patch");
}

async fn create_namespace(namespace: &str, client: Client, params: &PostParams) {
    let api: Api<Namespace> = Api::all(client);
    api.create(
        params,
        &Namespace {
            metadata: ObjectMeta {
                name: Some(namespace.to_string()),
                ..Default::default()
            },
            ..Default::default()
        }
    ).await.expect("should have been able to create a unique namespace; does it exist?");
    println!("created unique namespace: {namespace}");
}
