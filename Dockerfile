# Build Studio Dockerfile
# Multi-stage: build + runtime

FROM rust:1.77 as builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y libwebkit2gtk-4.0-dev libssl-dev build-essential curl npm nodejs pkg-config libgtk-3-dev librsvg2-dev
RUN cd src-tauri && cargo build --release && cd .. && npm install && npm run build

FROM debian:bookworm-slim as runtime
WORKDIR /app
COPY --from=builder /app/src-tauri/target/release/build-studio-cli /usr/local/bin/buildstudio-cli
COPY --from=builder /app/build /app/build
COPY --from=builder /app/builds /app/builds
COPY --from=builder /app/packages /app/packages
COPY --from=builder /app/plugins /app/plugins
COPY --from=builder /app/buildstudio.config.yaml /app/buildstudio.config.yaml
RUN apt-get update && apt-get install -y libwebkit2gtk-4.0-37 libssl3 libgtk-3-0 librsvg2-2 && rm -rf /var/lib/apt/lists/*

# CLI entrypoint
ENTRYPOINT ["buildstudio-cli"]
