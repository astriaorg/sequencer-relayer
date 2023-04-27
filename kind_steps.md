# preparation

## Install kind, kubectl

This assumes docker is available and running.

https://kind.sigs.k8s.io/docs/user/quick-start/

### Arch linux

```sh
# using rua to install from the AUR
$ rua install kind
$ sudo pacman -S kubectl
```

## start the kind cluster

```sh
$ kind create cluster --config containers/kind-cluster-config.yaml
$ kind get clusters
test-kind
$ kubectl config current-contetx
kind-test-kind
```

## deploy the nginx ingress provider and echo service

To route requests to the correct services, and to define a
default backend to be used when we don't yet have ingress rules
(these will be patched in when running tests):

```sh
$ kubectl apply -f \
	https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
$ kubectl apply -f \
	https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/docs/examples/http-svc.yaml
```

## pre-pull the images required to run the stack

This is called the single-use DaemonSet pattern:
https://jacobtomlinson.dev/posts/2023/quick-and-dirty-way-to-pre-pull-container-images-on-kubernetes/
https://codefresh.io/blog/single-use-daemonset-pattern-pre-pulling-images-kubernetes/

```sh
$ kubectl apply -f \
    templates/sequencer_relayer_stack_prepull.yaml
```

## deploy ingress

This does not yet contain any rules (since there aren't any):

```sh
$ kubectl apply -f \
    templates/sequencer_relayer_stack_ingress.yaml
```

# deploying a sequencer-relayer-stack

Deploying the sequencer relayer stack is as simple as:
```sh
$ kubectl apply -f \
    templates/sequencer_relayer_stack.yaml
```
This deploys a) the pod and its containers, and b) the kubernetes
service object that exposes the correct ports of that pod.
Finally and to make it accessible from outside kubernetes, one
must update the ingress rules so that requests get routed to
the service and finally the containers:
```sh
$ kubectl patch ingresses.networking.k8s.io \
    sequencer-relayer-ingress \
		--type='json' \
		--patch='[{
				"op": "add",
				"path": "/spec/rules/0",
				"value": {
					"http": {
						"paths":[
							{
								"pathType": "Prefix",
								"path": "/sequencer/?(.*)",
								"backend": {
									"service": {
										"name": "sequencer-relayer-stack-service",
										"port": { "number": 1318 }
									}
								}
							},
							{
								"pathType": "Prefix",
								"path": "/bridge/?(.*)",
								"backend": {
									"service": {
										"name": "sequencer-relayer-stack-service",
										"port": { "number": 26659 }
									}
								}
							}
					]}
				}
			}]'
```
The following command should then show the rule:
```sh
$ kubectl get ingress sequencer-relayer-ingress -o yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  annotations:
    kubectl.kubernetes.io/last-applied-configuration: |
      {"apiVersion":"networking.k8s.io/v1","kind":"Ingress","metadata":{"annotations":{"nginx.ingress.kubernetes.io/rewrite-target":"/$1"},"name":"sequencer-relayer-ingress","namespace":"default"},"spec":{"rules":[{"http":{"paths":[{"backend":{"service":{"name":"http-svc","port":{"number":80}}},"path":"/","pathType":"Prefix"}]}}]}}
    nginx.ingress.kubernetes.io/rewrite-target: /$1
  creationTimestamp: "2023-04-26T14:55:09Z"
  generation: 16
  name: sequencer-relayer-ingress
  namespace: default
  resourceVersion: "147097"
  uid: c2547e28-dbbc-4dab-99b4-9a3ec898a376
spec:
  rules:
  - http:
      paths:
      - backend:
          service:
            name: sequencer-relayer-stack-service
            port:
              number: 1318
        path: /sequencer/?(.*)
        pathType: Prefix
      - backend:
          service:
            name: sequencer-relayer-stack-service
            port:
              number: 26659
        path: /bridge/?(.*)
        pathType: Prefix
  - http:
      paths:
      - backend:
          service:
            name: http-svc
            port:
              number: 80
        path: /
        pathType: Prefix
status:
  loadBalancer:
    ingress:
    - hostname: localhost
```
And metro can be queried:
```sh
& curl 127.0.0.1/sequencer/cosmos/base/tendermint/v1beta1/blocks/latest
{
  "block_id": {
    "hash": "sSpfoa73oXQddYdskOz6VGG3a6hZ+MAYTMaiY41HfnQ=",
    "part_set_header": {
      "total": 1,
      "hash": "fl7xAVccz+b6dKwdPBIZHDSl4KN9l1jLIgb7hO0eMr0="
    }
  },
  "block": {
    "header": {
      "version": {
        "block": "11",
        "app": "0"
      },
      "chain_id": "test",
      "height": "1535",
      "time": "2023-04-27T14:35:01.432976256Z",
      "last_block_id": {
        "hash": "e8+lA66E21id0jLCSZKZbqlLR2USn0L09Nx2WoeloJ8=",
        "part_set_header": {
          "total": 1,
          "hash": "GHuPppYO548tjTD/o6y7Nm5IR/YT3MyTcjL+PRTaAg8="
        }
      },
      "last_commit_hash": "it3AwnT9mT/3pnajQ3x1v1k5EkHeihs1GCDvsYW5hj0=",
      "data_hash": "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=",
      "validators_hash": "U5hzFMsNK9ozjoCD5LKFVux3k4GiW5tmhKPoLCWx9oM=",
      "next_validators_hash": "U5hzFMsNK9ozjoCD5LKFVux3k4GiW5tmhKPoLCWx9oM=",
      "consensus_hash": "BICRvH3cKD93v7+R1zxE2ljD34qcvIZ0Bdi389qtoi8=",
      "app_hash": "Pq8dy53jfnFnFaPleK3o++k3ajMNpusN9bZn/Wc5oxY=",
      "last_results_hash": "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=",
      "evidence_hash": "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=",
      "proposer_address": "INaK2jlx5kaWGmLEq5dHYK4y6RI="
    },
    "data": {
      "txs": [
      ],
      "blobs": [
      ],
      "square_size": "0",
      "hash": "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU="
    },
    "evidence": {
      "evidence": [
      ]
    },
    "last_commit": {
      "height": "1534",
      "round": 0,
      "block_id": {
        "hash": "e8+lA66E21id0jLCSZKZbqlLR2USn0L09Nx2WoeloJ8=",
        "part_set_header": {
          "total": 1,
          "hash": "GHuPppYO548tjTD/o6y7Nm5IR/YT3MyTcjL+PRTaAg8="
        }
      },
      "signatures": [
        {
          "block_id_flag": "BLOCK_ID_FLAG_COMMIT",
          "validator_address": "INaK2jlx5kaWGmLEq5dHYK4y6RI=",
          "timestamp": "2023-04-27T14:35:01.432976256Z",
          "signature": "JBh/5ocUKFhje5E+ynersX5WYCOO4E8fTqH2LWrSCcqUgcZmkQOBFe1zmYjaoiYN7B3SiErurBmfy5S1VHuVDA=="
        }
      ]
    }
  },
  "sdk_block": {
    "header": {
      "version": {
        "block": "11",
        "app": "0"
      },
      "chain_id": "test",
      "height": "1535",
      "time": "2023-04-27T14:35:01.432976256Z",
      "last_block_id": {
        "hash": "e8+lA66E21id0jLCSZKZbqlLR2USn0L09Nx2WoeloJ8=",
        "part_set_header": {
          "total": 1,
          "hash": "GHuPppYO548tjTD/o6y7Nm5IR/YT3MyTcjL+PRTaAg8="
        }
      },
      "last_commit_hash": "it3AwnT9mT/3pnajQ3x1v1k5EkHeihs1GCDvsYW5hj0=",
      "data_hash": "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=",
      "validators_hash": "U5hzFMsNK9ozjoCD5LKFVux3k4GiW5tmhKPoLCWx9oM=",
      "next_validators_hash": "U5hzFMsNK9ozjoCD5LKFVux3k4GiW5tmhKPoLCWx9oM=",
      "consensus_hash": "BICRvH3cKD93v7+R1zxE2ljD34qcvIZ0Bdi389qtoi8=",
      "app_hash": "Pq8dy53jfnFnFaPleK3o++k3ajMNpusN9bZn/Wc5oxY=",
      "last_results_hash": "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=",
      "evidence_hash": "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=",
      "proposer_address": "metrovaloper1yrtg4k3ew8nyd9s6vtz2h968vzhr96gjad4ws4"
    },
    "data": {
      "txs": [
      ],
      "blobs": [
      ],
      "square_size": "0",
      "hash": "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU="
    },
    "last_commit": {
      "height": "1534",
      "round": 0,
      "block_id": {
        "hash": "e8+lA66E21id0jLCSZKZbqlLR2USn0L09Nx2WoeloJ8=",
        "part_set_header": {
          "total": 1,
          "hash": "GHuPppYO548tjTD/o6y7Nm5IR/YT3MyTcjL+PRTaAg8="
        }
      },
      "signatures": [
        {
          "block_id_flag": "BLOCK_ID_FLAG_COMMIT",
          "validator_address": "INaK2jlx5kaWGmLEq5dHYK4y6RI=",
          "timestamp": "2023-04-27T14:35:01.432976256Z",
          "signature": "JBh/5ocUKFhje5E+ynersX5WYCOO4E8fTqH2LWrSCcqUgcZmkQOBFe1zmYjaoiYN7B3SiErurBmfy5S1VHuVDA=="
        }
      ]
    }
  }
}
```
