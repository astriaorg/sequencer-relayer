default:
  @just --list

setup-cluster:
  kind create cluster --config ./test_environment/cluster-config.yml

delete-cluster:
  kind delete cluster --name test-cluster

deploy-ingress-controller:
  kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml

perform-prepull:
  kubectl apply -f ./test_environment/prepull-daemon-set.yml

create-namespace:
  kubectl create namespace test

deploy-test-environment:
  kubectl apply -n test -k ./test_environmnet/

hit-sequencer:
  curl http://test.localdev.me/sequencer/cosmos/base/tendermint/v1beta1/blocks/latest

wait-for-ingress-controller:
  kubectl wait --namespace ingress-nginx --for=condition=ready pod --selector=app.kubernetes.io/component=controller --timeout=600s

wait-for-prepull:
  kubectl wait --for=condition=ready pod --selector=name=sequencer-relayer-environment-prepull --timeout=600s
