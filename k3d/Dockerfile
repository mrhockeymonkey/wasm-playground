#FROM rancher/k3s:v1.24.6-k3s1

# extend the deislabs image which already includes spin & wws
FROM ghcr.io/deislabs/containerd-wasm-shims/examples/k3d:v0.11.1

# copy shims from runwasi project
ADD https://github.com/containerd/runwasi/releases/download/containerd-shim-wasmtime%2Fv0.4.0/containerd-shim-wasmtime-x86_64.tar.gz /tmp
RUN tar -xvzf /tmp/containerd-shim-wasmtime-x86_64.tar.gz -C /tmp &&\
cp /tmp/containerd-shim-wasmtime-v1 /bin


# add containerd config for wasmtime
COPY wasmtime-conf.txt /tmp
RUN cat /tmp/wasmtime-conf.txt >> /var/lib/rancher/k3s/agent/etc/containerd/config.toml.tmpl