FROM rustlang/rust:nightly as builder

WORKDIR /build

COPY . .

RUN cargo +nightly build --release --locked

FROM debian:stable

RUN apt-get update && apt-get install ca-certificates -y

COPY --from=builder /build/target/release/farm-operator /

ENTRYPOINT ["/farm-operator"]

ARG GITHUB_SHA
LABEL org.opencontainers.image.source="https://github.com/metalbear-co/farm-operator/tree/${GITHUB_SHA:-main}/"
