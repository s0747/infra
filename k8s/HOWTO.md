-Troubleshooting

```
sudo crictl ps | grep kube-apiserver
sudo journalctl -xeu kubelet --no-pager | tail -n 50

```

```
kubectl get events -n monitoring --sort-by='.metadata.creationTimestamp'
```
