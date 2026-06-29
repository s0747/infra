
```
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: lokalne-pvc
spec:
  accessModes:
    - ReadWriteOnce
  storageClassName: local-storage
  resources:
    requests:
      storage: 20Gi
```
