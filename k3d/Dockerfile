FROM rancher/k3s:v1.24.6-k3s1

# copy shims from target directory into the /bin
ADD https://github.com/containerd/runwasi/releases/download/containerd-shim-wasm/v0.1.2/containerd-shim-wasm-linux-amd64.tar.gz /tmp/
RUN gzip -d /tmp/containerd-shim-wasm-linux-amd64.tar.gz && \

#COPY ./.tmp /bin/
#tar -xvzf community_images.tar.gz -C some_custom_folder_name

# # copy in an containerd config into /etc/containerd/config.toml
# COPY config.toml.tmpl /var/lib/rancher/k3s/agent/etc/containerd/config.toml.tmpl
ENTRYPOINT [ "" ]
CMD [ "" ]