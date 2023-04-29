FROM --platform=$BUILDPLATFORM mcr.microsoft.com/dotnet/sdk:7.0 as build
WORKDIR /opt/build
COPY . .
RUN dotnet restore
RUN dotnet build -c Release --output ./dist

FROM scratch
WORKDIR /opt/app
COPY --from=build /opt/build/dist/cs-hello.wasm ./cs-hello.wasm
ENTRYPOINT [ "cs-hello.wasm" ]