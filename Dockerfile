FROM --platform=$BUILDPLATFORM instrumentisto/rust:nightly as builder
ARG TARGETARCH

WORKDIR /build

RUN curl -sSL https://bootstrap.pypa.io/get-pip.py -o get-pip.py && python3 get-pip.py
RUN python3 -m pip install ziglang
RUN cargo install cargo-zigbuild

COPY ./platform.sh ./rust-toolchain.toml .

RUN chmod +x ./platform.sh
RUN ./platform.sh

RUN rustup component add --toolchain nightly rustfmt
RUN rustup target add --toolchain nightly $(cat /.platform)
RUN apt-get update && apt-get install -y $(cat /.compiler)

COPY . .

RUN cargo +nightly zigbuild -p farm-operator --target $(cat /.platform) --release --locked

RUN cp /build/target/$(cat /.platform)/release/farm-operator /farm-operator

FROM debian:stable

RUN apt-get update && apt-get install ca-certificates -y

COPY --from=builder /farm-operator /

ENTRYPOINT ["/farm-operator"]

ARG GITHUB_SHA
LABEL org.opencontainers.image.source="https://github.com/metalbear-co/farm-operator/tree/${GITHUB_SHA:-main}/"
