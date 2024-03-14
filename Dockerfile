# ビルドステージ
FROM arm64v8/rust:latest AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release --package stats-manager

# 実行ステージ
FROM arm64v8/debian:buster-slim
COPY --from=builder /usr/src/myapp/target/release/stats-manager /usr/local/bin/stats-manager
EXPOSE 2784
CMD ["stats-manager"]
