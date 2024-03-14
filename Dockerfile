# Rustの公式イメージをベースにする。アーキテクチャを指定していないが、ビルドシステムが適切なイメージを選択する。
FROM rust:latest as builder

RUN rustup target add armv7-unknown-linux-musleabihf
RUN apt-get update && apt-get install -y musl-tools gcc-arm-linux-gnueabihf
WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release --target=armv7-unknown-linux-musleabihf

FROM arm32v7/debian:bullseye-slim
COPY --from=builder /usr/src/myapp/target/armv7-unknown-linux-musleabihf/release/stats-manager /usr/local/bin/stats-manager
EXPOSE 2784
CMD ["stats-manager"]
