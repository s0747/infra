kubectl get pods -n kube-system -l k8s-app=cilium
kubectl exec -n kube-system cilium-6nmsj -c cilium-agent -- cilium status

kubectl get leases -n kube-system | grep cilium

kubectl edit cm cilium-config -n kube-system

data:
  enable-l2-announcements: "true"
  enable-lb-ipam: "true"
