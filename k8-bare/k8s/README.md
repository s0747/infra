# K8S-BARE-METAL


- PREPARE
```
# Kubernetes

# apt-transport-https may be a dummy package; if so, you can skip that package
sudo apt-get install -y apt-transport-https ca-certificates curl gnupg

# If the folder `/etc/apt/keyrings` does not exist, it should be created before the curl command, read the note below.
# sudo mkdir -p -m 755 /etc/apt/keyrings
curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.36/deb/Release.key | sudo gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg
sudo chmod 644 /etc/apt/keyrings/kubernetes-apt-keyring.gpg # allow unprivileged APT programs to read this keyring

# This overwrites any existing configuration in /etc/apt/sources.list.d/kubernetes.list
echo 'deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.36/deb/ /' | sudo tee /etc/apt/sources.list.d/kubernetes.list
sudo chmod 644 /etc/apt/sources.list.d/kubernetes.list   # helps tools such as command-not-found to work correctly
sudo apt-get update
sudo apt-get install -y kubectl

# Helm
```
sudo apt update
sudo apt install helm
 lub
curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/master/scripts/get-helm-3
chmod 700 get_helm.sh
./get_helm.sh
```

# cgroups
stat -fc %T /sys/fs/cgroup/
```

# DOWNLOAD K8S
```
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl.sha256"
echo "$(cat kubectl.sha256)  kubectl" | sha256sum --check
sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
kubectl version --client
```
# OR
```
sudo apt-get install -y apt-transport-https ca-certificates curl gnupg
# If the folder `/etc/apt/keyrings` does not exist, it should be created before the curl command, read the note below.
# sudo mkdir -p -m 755 /etc/apt/keyrings
curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.36/deb/Release.key | sudo gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg
sudo chmod 644 /etc/apt/keyrings/kubernetes-apt-keyring.gpg # allow unprivileged APT programs to read this keyring

#This overwrites any existing configuration in /etc/apt/sources.list.d/kubernetes.list
echo 'deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.36/deb/ /' | sudo tee /etc/apt/sources.list.d/kubernetes.list
sudo chmod 644 /etc/apt/sources.list.d/kubernetes.list   # helps tools such as command-not-found to work correctly
sudo apt-get update
sudo apt-get install -y kubectl kubeadm
kubectl version --client
```

# REMOVE K8S
```
sudo systemctl stop kubelet containerd docker

df -h | grep kubelet | awk '{print $6}' | xargs -r sudo umount -l

sudo rm -rf /etc/kubernetes/
sudo rm -rf /var/lib/kubelet/
sudo rm -rf /var/lib/etcd/
sudo rm -rf /var/lib/cni/
sudo rm -rf /etc/cni/net.d/
sudo rm -rf /var/run/kubernetes/
rm -rf $HOME/.kube/
find /var/log/pods/ -name "*.log" -exec truncate -s 0 {} \;

```

- Cluster reset 
```
sudo kubeadm reset -f

sudo crictl --runtime-endpoint unix:///var/run/containerd/containerd.sock rmi --prune

sudo ctr -n k8s.io images list

sudo ctr -n k8s.io images prune -all
```

# INIT
```
sudo systemctl start kubelet containerd 
sudo kubeadm init --config k8s/kubeadm-config.yaml

kubectl get nodes
kubectl describe node <nazwa-węzła> | grep Taints
kubectl get pods -n kube-system
kubectl cluster-info
```

# unTaint
```
kubectl taint nodes --all node-role.kubernetes.io/control-plane:NoSchedule-
``

# Flannel:


- GET CONFIG TO HOME DIR
```
mkdir -p $HOME/.kube
sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config
```

- K8S status
```
kubectl get pods -n kube-system -l k8s-app=kube-dns
kubectl get pods -n ingress-nginx


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
