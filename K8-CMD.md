
# ALL
kubectl get all -n <NS>
lub
kubectl api-resources --namespaced=true -o name | xargs -n 1 kubectl get --show-kind --ignore-not-found -n <NS>

# LABELS
kubectl get pods --show-labels -n <nazwa-namespace>
kubectl get pods -l app=frontend -n <nazwa-namespace>
kubectl get pods -l app=frontend,env=production -n <nazwa-namespace>
kubectl get pods -l 'env in (staging, production)' -n <nazwa-namespace>
kubectl get pods -L env,version -n <nazwa-namespace>

# LISTA TYPOW OBIEKTOW i SKROTOW
kubectl api-resources 

kubectl get events -n k8s-wprawki --field-selector reason=Unhealthy
