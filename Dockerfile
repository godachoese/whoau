FROM rust:1.73-alpine as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.14
LABEL author="Goda <goda.choe@tridge.com>"
ENV WHOAU_PORT=9999
WORKDIR /app
COPY --from=builder /app/target/release/whoau .
ENTRYPOINT ["./whoau"]