
# PRE INSTALL
```
sudo mount bpffs /sys/fs/bpf -t bpf

# and add to fstab:

bpffs                  /sys/fs/bpf             bpf     defaults        0 0
```

# Cilium INSTALL
```
helm repo add cilium https://helm.cilium.io/
helm repo update

cilium install --set ipam.mode=cluster-pool
kubectl scale deployment cilium-operator --replicas=1 -n kube-system

lub 
helm install cilium cilium/cilium 
 --namespace kube-system \
 -f values.yml

# Cilium status
```
cilium status --wait
cilium connectivity test
```

# 1. Usuń DaemonSet, żeby kontenery kube-proxy przestały działać
kubectl -n kube-system delete ds kube-proxy

# 2. Usuń ConfigMapę (zapobiegnie to ponownemu zainstalowaniu podczas aktualizacji klastra)
kubectl -n kube-system delete cm kube-proxy

kubectl -n kube-system exec ds/cilium -- cilium-dbg status | grep KubeProxyReplacement
KubeProxyReplacement:   True   [eth0 (Direct Routing)]

# Cilium Restart
kubectl rollout restart daemonset cilium -n kube-system

# Test
kubectl -n kube-system exec ds/cilium -- cilium status | grep KubeProxyReplacement
# Powinno być 
# "KubeProxyReplacement:    True"

# Logi
kubectl logs -n kube-system -l k8s-app=cilium --tail=500 | grep -i "kube-proxy"

# Test LB - jesli jest
cilium hubble observe --to-ip 192.168.8.200 -f

# Cilium jako LoadBalancer
--- values-cilium.yaml
l2announcements:
  enabled: true
---

kubectl -f values-cilium-ip-pool.yaml
kubectl get ciliumloadbalancerippools
kubectl apply -f test-lb.yaml
kubectl get svc nginx-loadbalancer
```

# Do SPRAWDZENIA
```
kubectl get pods -n kube-system -l k8s-app=cilium
kubectl exec -n kube-system cilium-6nmsj -c cilium-agent -- cilium status

kubectl get leases -n kube-system | grep cilium

kubectl edit cm cilium-config -n kube-system

data:
  enable-l2-announcements: "true"
  enable-lb-ipam: "true"


```
