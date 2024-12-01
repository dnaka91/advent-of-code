# syntax=docker/dockerfile:1.7-labs
FROM rust:1.82 as builder

WORKDIR /volume

RUN apt-get update && \
    apt-get install -y --no-install-recommends musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo init --bin

COPY --parents benches/ Cargo.lock Cargo.toml ./

RUN cargo build --release --target x86_64-unknown-linux-musl

COPY src/ src/

RUN touch src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3 as newuser

RUN echo "aoc:x:1000:" > /tmp/group && \
    echo "aoc:x:1000:1000::/dev/null:/sbin/nologin" > /tmp/passwd

FROM scratch

COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/aoc /bin/
COPY --from=newuser /tmp/group /tmp/passwd /etc/

USER aoc

ENTRYPOINT ["/bin/aoc"]
