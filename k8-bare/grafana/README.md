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
