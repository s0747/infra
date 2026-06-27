# DOCKER BUILDX (multiarch)
# Tworzymy nowy builder obsługujący wiele platform
docker buildx create --name multi-builder --use

# Uruchamiamy
docker buildx inspect --bootstrap

# Sprawdzamy
docker buildx ls

# Budujemy Multiarch i push
docker buildx build --no-cache --platform linux/amd64,linux/arm64 \
 --label "org.opencontainers.image.source=https://github.com/s0747/infra" \
 --label "org.opencontainers.image.description=myapp container image" \
 --label "org.opencontainers.image.licenses=MIT" \
 -t ghcr.io/s0747/myapp:0.1.1-multi --push .
# LUB NATIVE i push
docker buildx build --no-cache \
 --label "org.opencontainers.image.source=https://github.com/s0747/infra" \
 --label "org.opencontainers.image.description=myapp container image" \
 --label "org.opencontainers.image.licenses=MIT" \
 -t ghcr.io/s0747/myapp:0.1.1 --push .

# LUB LOKALNIE
docker buildx build --load -t myapp:local .
docker run -d -p 8080:8080 --name myapp myapp:local

# Sprawdzamy
docker run -d -p 8080:8080 --name myapp ghcr.io/s0747/myapp:0.1.1
# lub
docker run -d -p 8080:8080 --name myapp ghcr.io/s0747/myapp:0.1.0-multi
curl http://localhost:8080/metrics
docker stop test-app && docker rm myapp

# OR OLD BUILD and local image only
# Build
docker buildx build -t myapp:0.1.0 .

# Save to tar
docker save myapp:0.1.0> /tmp/myapp:0.1.0.tar

# Import do k8s
sudo ctr -n=k8s.io images import /tmp/myapp:0.1.0.tar

# Clean UP:
rm /tmp/myapp:v1.2.tar

# Sprawdzenie czy jest w lokalnym rejestrze k8s
sudo ctr -n=k8s.io images list | grep myapp:0.1.0

# CLEAN UP:
docker image rm myapp:0.1.0
sudo ctr -n=k8s.io images rm docker.io/library/myapp:0.1.0

----
# DEBUG:
kubectl debug -it myapp-pod --image=busybox --target=myapp-container
# Wejdź do środka kontenera przez proces PID 1
cd /proc/1/root/app

# Zobacz, jak nazywa się plik
ls -l

----
