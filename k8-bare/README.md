K8S - NOTES

- Prepare new machine
```
#cgroups

stat -fc %T /sys/fs/cgroup/
```
- Cluster reset 
```
sudo kubeadm reset -f
sudo crictl --runtime-endpoint unix:///var/run/containerd/containerd.sock rmi --prune

sudo ctr -n k8s.io images list

sudo ctr -n k8s.io images clear

sudo kubeadm init --config kubeadm-config.yaml
```

-Get config
```
mkdir -p $HOME/.kube
sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config
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
#
cilium install --set ipam.operator.clusterPoolIPv4PodCIDRList="10.244.0.0/16"
```
- Cilium status
```
cilium status
```


HELM Cilium install
```
helm repo add cilium https://helm.cilium.io/
helm repo update

helm upgrade cilium cilium/cilium --version 1.19.3 \
  --namespace kube-system \
  --reuse-values \
  --set hubble.relay.enabled=true \
  --set kubeProxyReplacement=true \
  --set bpf.masquerade=true \
  --set ipam.mode=kubernetes \
  --set multiPoolL4LB=true \
  --set l2announcements.enabled=true \
  --set gatewayAPI.enabled=true \ 

# Restart
kubectl rollout restart daemonset cilium -n kube-system
# Test
kubectl -n kube-system exec ds/cilium -- cilium status | grep KubeProxyReplacement
# Powinno być 
# "KubeProxyReplacement:    True"

# Logi
kubectl logs -n kube-system -l k8s-app=cilium --tail=500 | grep -i "kube-proxy"

# Test LB - jesli jest
cilium hubble observe --to-ip 192.168.8.200 -f
```

- K8S status
```
kubectl get pods -n kube-system -l k8s-app=kube-dns
kubectl get pods -n ingress-nginx

- Ingress
```
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/baremetal/deploy.yaml
lub
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm repo update
helm install moj-ingress ingress-nginx/ingress-nginx \
  --namespace ingress-nginx --create-namespace \
  --set controller.hostNetwork=true \
  --set controller.service.type=ClusterIP
```

- MetalLB
```
kubectl apply -f https://raw.githubusercontent.com/metallb/metallb/v0.14.8/config/manifests/metallb-native.yaml

kubectl apply -f metallb-config.yaml

kubectl apply -f nginx-service.yaml

kubectl get svc -n ppa-ns
```

-LocalStorage
```
kubectl apply -f https://raw.githubusercontent.com/rancher/local-path-provisioner/v0.0.30/deploy/local-path-storage.yaml
kubectl get storageclass
kubectl get configmap local-path-config -n local-path-storage -o jsonpath='{.data.config\.json}'

```

- Grafana, prometheus
```
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo update

helm install monitoring Prometheus-community/Kube-Prometheus-stack \
--namespace monitoring \
--values grafana-values.yaml

```

- Loki
```
helm repo add grafana https://grafana.github.io/helm-charts
helm repo update

helm install loki grafana/loki-stack \
--namespace monitoring \
  --values loki-values.yaml
```
