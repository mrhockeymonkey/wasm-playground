FROM mcr.microsoft.com/dotnet/sdk:7.0 as build
WORKDIR /opt/build
COPY . .
RUN dotnet restore
RUN dotnet build -c Release --output ./dist

FROM scratch
# WORKDIR /opt/app
COPY --from=build /opt/build/dist/WasiConsole.wasm ./bin/WasiConsole.wasm
COPY --from=build /opt/build/spin.toml .
# CMD [ "WasiConsole.wasm" ]