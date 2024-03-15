FROM rust:1.69 as builder

RUN apt-get update && apt-get install -y \
  gcc-arm-linux-gnueabihf \
  libc6-dev

RUN rustup target add armv7-unknown-linux-gnueabihf

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build --release --target=armv7-unknown-linux-gnueabihf

FROM arm32v7/ubuntu:20.04
COPY --from=builder /usr/src/myapp/target/armv7-unknown-linux-gnueabihf/release/stats-manager /usr/local/bin/
EXPOSE 2784
CMD ["stats-manager"]