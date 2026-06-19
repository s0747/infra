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

- BPF
```
sudo mount bpffs /sys/fs/bpf -t bpf

add to fstab:

bpffs                  /sys/fs/bpf             bpf     defaults        0 0
```

- Cilium install
```
cilium install --set ipam.operator.clusterPoolIPv4PodCIDRList="10.244.0.0/16"
```
- Cilium status
```
cilium status
```
- K8S status
```
kubectl get pods -n kube-system -l k8s-app=kube-dns
kubectl get pods -n ingress-nginx
```

- Ingress
```
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/baremetal/deploy.yaml
```


- Grafana, prometheus
```
helm repo add prometheus-community    https://prometheus-community.github.io/helm-charts
helm repo update

helm install  grafana/loki-stack \
  --namespace monitoring \
  --values loki-values.yaml

```

- Loki
```
helm repo add grafana https://grafana.github.io/helm-charts
helm repo update

helm install loki grafana/loki-stack \
  --namespace monitoring \
  --values loki-values.yaml
```
