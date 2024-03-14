# Rustの公式イメージをベースにする。アーキテクチャを指定していないが、ビルドシステムが適切なイメージを選択する。
FROM rust:latest as builder

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# アプリケーションのソースコードをコピー
COPY . .

# ターゲットアーキテクチャ向けにクロスコンパイルを設定
RUN rustup target add aarch64-unknown-linux-gnu
RUN apt-get update && apt-get install -y crossbuild-essential-arm64
RUN cargo build --release --target aarch64-unknown-linux-gnu --package stats-manager

# 実行ステージ
FROM debian:buster-slim
COPY --from=builder /usr/src/myapp/target/aarch64-unknown-linux-gnu/release/stats-manager /usr/local/bin/stats-manager
EXPOSE 2784
CMD ["stats-manager"]
