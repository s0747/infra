# Build
docker build -t myapp:v1.2 .

# Save to tar
docker save myapp:v1.2> /tmp/myapp:v1.2.tar

# Import do k8s
sudo ctr -n=k8s.io images import /tmp/myapp:v1.2.tar

# Clean UP:
rm /tmp/myapp:v1.2.tar

# Sprawdzenie czy jest w lokalnym rejestrze k8s
sudo ctr -n=k8s.io images list | grep myapp:v1.2

#CLEAN UP:
docker image rm myapp:v1.2
sudo ctr -n=k8s.io images rm docker.io/library/myapp:v1.2


#DEBUG:
kubectl debug -it myapp-pod --image=busybox --target=myapp-container
