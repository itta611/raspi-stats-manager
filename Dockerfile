FROM ubuntu:20.04 as builder

RUN apt-get update && apt-get install -y \
	build-essential \
  gcc-arm-linux-gnueabihf \
	curl

RUN useradd -ms /bin/bash local
USER local
WORKDIR /home/local

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENV PATH="/home/local/.cargo/bin:${PATH}"

WORKDIR /home/local/myapp

RUN rustup target add armv7-unknown-linux-gnueabihf

COPY . .
RUN cargo build --release --target=armv7-unknown-linux-gnueabihf


FROM ubuntu:20.04
COPY --from=builder /home/local/myapp/target/armv7-unknown-linux-gnueabihf/release/stats-manager /usr/local/bin/stats-manager
EXPOSE 2784
CMD ["stats-manager"]
