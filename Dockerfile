FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/stats-manager .
EXPOSE 2784
CMD [ "./stats-manager" ]
