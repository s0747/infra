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
 -t ghcr.io/s0747/k8-app:0.1.1-multi --push .
 
# LUB NATIVE i push
docker buildx build --no-cache \
 --label "org.opencontainers.image.source=https://github.com/s0747/infra" \
 --label "org.opencontainers.image.description=myapp container image" \
 --label "org.opencontainers.image.licenses=MIT" \
 -t ghcr.io/s0747/k8-app:0.1.1 --push .

# LUB LOKALNIE
docker buildx build --load -t k8-app:local .
docker run -d -p 8080:8080 --name k8-app k8-app:local

# Sprawdzamy
docker run -d -p 8080:8080 --name k8-app ghcr.io/s0747/k8-app:0.1.1
# lub
docker run -d -p 8080:8080 --name k8-app ghcr.io/s0747/k8-app:0.1.1-multi
curl http://localhost:8080/metrics
docker stop k8-app && docker rm k8-app

# OR OLD BUILD and local image only
# Build
docker buildx build -t k8-app:0.1.1 .

# Save to tar
docker save k8-app:0.1.0> /tmp/k8-app:0.1.1.tar

# Import do k8s
sudo ctr -n=k8s.io images import /tmp/k8-app:0.1.1.tar

# Clean UP:
rm /tmp/k8-app:0.1.1.tar

# Sprawdzenie czy jest w lokalnym rejestrze k8s
sudo ctr -n=k8s.io images list | grep k8-app:0.1.1

# CLEAN UP:
docker image rm k8-app:0.1.1
sudo ctr -n=k8s.io images rm docker.io/library/k8-app:0.1.1

----
# DEBUG:
kubectl debug -it k8-app --image=busybox --target=myapp-container
# Wejdź do środka kontenera przez proces PID 1
cd /proc/1/root/k8-app

# Zobacz, jak nazywa się plik
ls -l

----
