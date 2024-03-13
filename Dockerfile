FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release --package stats-reporter
RUN cargo build --release --package stats-manager

FROM alpine:latest as stats-reporter
WORKDIR /app
COPY --from=builder /app/target/release/stats-reproter .
EXPOSE 2784
CMD [ "./stats-reporter" ]

FROM alpine:latest as stats-manager
WORKDIR /app
COPY --from=builder /app/target/release/stats-manager .
EXPOSE 2784
CMD [ "./stats-manager" ]