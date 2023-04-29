# K3D

https://nigelpoulton.com/webassembly-on-kubernetes-ultimate-hands-on/
https://github.com/deislabs/containerd-wasm-shims/tree/main/deployments/k3d

```powershell
$k3d = Get-Command C:\ProgramData\k3d\k3d.exe
& $k3d cluster create wasm-cluster --image ghcr.io/deislabs/containerd-wasm-shims/examples/k3d:v0.5.1 -p "8081:80@loadbalancer" --agents 2

kubectl get nodes

kubectl label nodes k3d-wasm-cluster-agent-0 spin=yes

kubectl apply -f .\runtime-class.yaml
#runtimeclass.node.k8s.io/spin-test created

kubectl get runtimeclass
# NAME        HANDLER   AGE
# spin-test   spin      53s

kubectl apply -f .\rust-hello.yaml
# deployment.apps/wasm-spin created
# service/wasm-spin created
# middleware.traefik.containo.us/strip-prefix created
# ingress.networking.k8s.io/wasm-ingress created

iwr http://127.0.0.1:8081/spin/hello | select content

# Content
# -------
# Hello world from Spin!
```