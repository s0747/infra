# Helm
```
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm repo update

# Install
```
helm install ingress-nginx ingress-nginx/ingress-nginx \
  --namespace ingress-nginx \
  --create-namespace
```
# Sprawdzenie
```
kubectl get svc -n ingress-nginx
```

# Przykład:
```
kubectl apply -f ingress-test.yaml
```

vi /etc/hosts
```
192.168.1.192  moja-aplikacja.local
```
