# Webassembly on K3D 

Extends the deislabs k3d image to include wasmtime from containerd/runwasi 

```bash
docker build -t myk3d .

# create a wasm enabled k8c cluster
k3d cluster create my-wasm-cluster --image myk3d -p "8081:80@loadbalancer" --agents 2

kubectl get nodes
NAME                           STATUS   ROLES                  AGE   VERSION
k3d-my-wasm-cluster-agent-0    Ready    <none>                 16s   v1.27.8+k3s2
k3d-my-wasm-cluster-server-0   Ready    control-plane,master   20s   v1.27.8+k3s2
k3d-my-wasm-cluster-agent-1    Ready    <none>                 16s   v1.27.8+k3s2

kubectl apply -f runtime-class.yaml

# test wasmtime
kubectl apply -f deployment-wasmtime.yaml
kubectl logs rust-example-hello-5dc474b79b-9x8lv
#Hello WasmEdge!

# test spin
kubectl apply -f deployment-spin.yaml
curl http://127.0.0.1:8081/spin/hello 
#Hello world from Spin!

# test wws



```

https://github.com/deislabs/containerd-wasm-shims