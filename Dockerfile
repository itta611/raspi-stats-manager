# Rustの公式イメージをベースにする。アーキテクチャを指定していないが、ビルドシステムが適切なイメージを選択する。
FROM rust:1.58 as builder

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# 依存関係のファイルをコピーし、依存関係をビルド
COPY . ./
RUN mkdir src/ && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/stats_manager*

# アプリケーションのソースコードをコピー
COPY . .

# リリースビルド
RUN cargo install --path .

# 実行ステージ
FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/stats-manager /usr/local/bin/stats-manager

EXPOSE 2784
CMD ["stats-manager"]
