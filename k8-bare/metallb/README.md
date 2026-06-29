helm repo add metallb https://metallb.github.io/metallb
helm repo update

helm install metallb metallb/metallb \
  --namespace metallb-system \
  --create-namespace

kubectl apply -f metallb-pool.yaml
kubectl apply -f metallb-l2.yaml

#sprawdzenie
kubectl get pods -n metallb-system
