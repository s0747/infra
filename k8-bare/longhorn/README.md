sudo apt update
sudo apt install -y open-iscsi nfs-common
sudo systemctl enable --now iscsid


```
helm repo add longhorn https://charts.longhorn.io
helm repo update
```

```
helm install longhorn longhorn/longhorn \
  --namespace longhorn-system \
  --create-namespace \
  --version 1.8.0
```

# Ingress dla dashboard
```
kubectl apply -f longhorn-ingress.yaml
```

# Przykład
```
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: automatyczny-dysk-longhorn
spec:
  accessModes:
    - ReadWriteOnce
  storageClassName: longhorn # <-- Wskazujesz klasę zainstalowaną przez Helm
  resources:
    requests:
      storage: 10Gi # Prosisz o 10 GB przestrzeni
```
