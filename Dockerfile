FROM rust:1.39-alpine as builder

WORKDIR /app
COPY src/ src/
COPY Cargo.lock Cargo.toml ./

RUN cargo install --path .


FROM alpine:3.10

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/aoc /app/

ENTRYPOINT ["./aoc"]
