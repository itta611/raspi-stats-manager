# Rustの公式イメージをベースにする。アーキテクチャを指定していないが、ビルドシステムが適切なイメージを選択する。
FROM rust:1.58 as builder

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# 依存関係のファイルをコピーし、依存関係をビルドするためのダミービルド
COPY Cargo.toml Cargo.lock ./
RUN mkdir src/ && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src/ && rm -f target/release/deps/stats_manager*

# 実際のアプリケーションのソースコードをコピー
COPY . .

# アプリケーションのビルド
RUN cargo build --release

# 実行ステージ
FROM debian:buster-slim
COPY --from=builder /usr/src/myapp/target/release/stats-manager /usr/local/bin/stats-manager

CMD ["./stats-manager"]
