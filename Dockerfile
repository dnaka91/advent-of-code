FROM rust:1.63 as builder

WORKDIR /volume

RUN apt-get update && \
    apt-get install -y --no-install-recommends musl-tools=1.2.2-1 && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo init --bin

COPY Cargo.lock Cargo.toml ./

RUN cargo build --release --target x86_64-unknown-linux-musl

COPY benches/ benches/
COPY src/ src/

RUN touch src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.16 as newuser

RUN echo "aoc:x:1000:" > /tmp/group && \
    echo "aoc:x:1000:1000::/dev/null:/sbin/nologin" > /tmp/passwd

FROM scratch

COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/aoc /bin/
COPY --from=newuser /tmp/group /tmp/passwd /etc/

USER aoc

ENTRYPOINT ["/bin/aoc"]
