FROM rust:1.56-alpine as builder

WORKDIR /volume

RUN apk add --no-cache musl-dev=~1.2

COPY benches/ benches/
COPY src/ src/
COPY Cargo.lock Cargo.toml ./

RUN cargo build --release && \
    strip --strip-all target/release/aoc

FROM alpine:3.14 as newuser

RUN echo "aoc:x:1000:" > /tmp/group && \
    echo "aoc:x:1000:1000::/dev/null:/sbin/nologin" > /tmp/passwd

FROM scratch

COPY --from=builder /volume/target/release/aoc /bin/
COPY --from=newuser /tmp/group /tmp/passwd /etc/

USER aoc

ENTRYPOINT ["/bin/aoc"]
