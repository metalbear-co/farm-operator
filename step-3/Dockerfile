FROM --platform=$BUILDPLATFORM instrumentisto/rust:nightly as builder
ARG TARGETARCH

WORKDIR /build

COPY ./platform.sh ./rust-toolchain.toml .

RUN ./platform.sh
RUN rustup component add --toolchain nightly rustfmt
RUN rustup target add --toolchain nightly $(cat /.platform)
RUN apt-get update && apt-get install -y $(cat /.compiler)

COPY . .

RUN cargo +nightly build -p farm-operator-3 --target $(cat /.platform) --release --locked

RUN cp /build/target/$(cat /.platform)/release/farm-operator-3 /farm-operator-3

FROM debian:stable

RUN apt-get update && apt-get install ca-certificates -y

COPY --from=builder /farm-operator-3 /

ENTRYPOINT ["/farm-operator-3"]

ARG GITHUB_SHA
LABEL org.opencontainers.image.source="https://github.com/metalbear-co/farm-operator/tree/${GITHUB_SHA:-main}/"
