K8S - NOTES

- Reset 
```
sudo kubeadm reset -f
sudo crictl --runtime-endpoint unix:///var/run/containerd/containerd.sock rmi --prune

sudo ctr -n k8s.io images list

sudo ctr -n k8s.io images clear

sudo kubeadm init --config kubeadm-config.yaml
```

- Get Cilium
```
CILIUM_CLI_VERSION=$(curl -s https://raw.githubusercontent.com/cilium/cilium-cli/main/stable.txt)
CLI_ARCH=amd64
if [ "$(uname -m)" = "aarch64" ]; then CLI_ARCH=arm64; fi
curl -L --fail --remote-name-all https://github.com/cilium/cilium-cli/releases/download/${CILIUM_CLI_VERSION}/cilium-linux-${CLI_ARCH}.tar.gz{,.sha256sum}
sha256sum --check cilium-linux-${CLI_ARCH}.tar.gz.sha256sum
sudo tar xzvfC cilium-linux-${CLI_ARCH}.tar.gz /usr/local/bin
rm cilium-linux-${CLI_ARCH}.tar.gz{,.sha256sum}

```

- Cilium install
```
cilium install
```
- Cilium status
```
clilium status
```
- K8S status
```
kubectl get pods -n kube-system -l k8s-app=kube-dns
```
