helm repo add flannel https://flannel-io.github.io/flannel
helm install my-flannel flannel/flannel

helm install flannel flannel/flannel \
  --namespace kube-system \
  --set podCidr="10.244.0.0/16"

#Sprawdzenie
```
kubectl get pods -n kube-system -l app=flannel

```
