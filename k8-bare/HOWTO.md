-Troubleshooting

```
sudo crictl ps | grep kube-apiserver
sudo journalctl -xeu kubelet --no-pager | tail -n 50

kubectl get events -n monitoring --sort-by='.metadata.creationTimestamp'

# Logi scheduler
kubectl logs -n kube-system -l component=kube-scheduler

# Pody oczekujące na utworzenie
kubectl get pods -A --field-selector status.phase=Pending
```
