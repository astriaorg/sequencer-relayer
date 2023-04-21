use eyre::WrapErr as _;
use jsonrpsee_core::{server::IntoResponse, RpcResult};
use jsonrpsee_server::{RpcModule, ServerBuilder, ServerHandle};
use jsonrpsee_types::ResponsePayload;
use parking_lot::Mutex;
use std::sync::Arc;

use crate::relayer::Relayer;

pub struct RpcServer {
    server_handle: ServerHandle,
}

type HealthFn = fn(&Relayer) -> (u64, u64);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct HealthResponse {
    pub curr_sequencer_height: u64,
    pub curr_da_height: u64,
}

impl IntoResponse for HealthResponse {
    type Output = HealthResponse;

    fn into_response(self) -> ResponsePayload<'static, Self::Output> {
        ResponsePayload::result(self)
    }
}

impl RpcServer {
    pub async fn new(
        listen_addr: &str,
        listen_port: u16,
        health_fn: HealthFn, // impl Fn() -> (u64, u64) + Send + Sync + 'static,
        state: Arc<Mutex<Relayer>>,
    ) -> eyre::Result<RpcServer> {
        let addrs: &[std::net::SocketAddr] =
            &[format!("{}:{}", listen_addr, listen_port).parse().unwrap()];

        let server = ServerBuilder::default()
            .build(addrs)
            .await
            .wrap_err("failed to build RpcServer")?;

        let mut module = RpcModule::new(());
        module.register_method::<RpcResult<HealthResponse>, _>("system_health", move |_, _| {
            let (curr_sequencer_height, curr_da_height) = health_fn(&state.lock());
            Ok(HealthResponse {
                curr_sequencer_height,
                curr_da_height,
            })
        })?;

        let server_handle = server.start(module)?;
        Ok(RpcServer { server_handle })
    }

    pub async fn stop(self) -> eyre::Result<()> {
        self.server_handle
            .stop()
            .wrap_err("failed to stop RpcServer")
    }
}
