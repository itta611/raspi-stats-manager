# Rustの公式イメージをベースにする。アーキテクチャを指定していないが、ビルドシステムが適切なイメージを選択する。
FROM rust:latest as builder

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# アプリケーションのソースコードをコピー
COPY . .

# リリースビルド
RUN cargo build --release --package stats-manager

# 実行ステージ
FROM debian:buster-slim
COPY --from=builder /usr/src/myapp/target/release/stats-manager /usr/local/bin/stats-manager

EXPOSE 2784
CMD ["stats-manager"]
