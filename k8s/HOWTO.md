-Troubleshooting

```
sudo crictl ps | grep kube-apiserver
sudo journalctl -xeu kubelet --no-pager | tail -n 50

```
