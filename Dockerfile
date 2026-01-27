FROM rust:1 AS base

RUN apt-get update -y

WORKDIR /project
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src ./src

RUN mkdir -p /out/bin

FROM base AS build-arm64
RUN apt-get install gcc-aarch64-linux-gnu -y
RUN rustup target add aarch64-unknown-linux-musl
RUN CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=/usr/bin/aarch64-linux-gnu-gcc \
    cargo build \
    --release \
    --target=aarch64-unknown-linux-musl

RUN cp ./target/aarch64-unknown-linux-musl/release/ocicp /out/bin/ocicp-linux-arm64
RUN cp /out/bin/ocicp-linux-arm64 /out/bin/ocicp

FROM base AS build-amd64
RUN apt-get install gcc-x86-64-linux-gnu -y
RUN rustup target add x86_64-unknown-linux-musl
RUN CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=/usr/bin/x86_64-linux-gnu-gcc \
    cargo build \
    --release \
    --target=x86_64-unknown-linux-musl

RUN cp ./target/x86_64-unknown-linux-musl/release/ocicp /out/bin/ocicp-linux-amd64
RUN cp /out/bin/ocicp-linux-amd64 /out/bin/ocicp

ARG TARGETARCH
FROM build-$TARGETARCH AS final

FROM scratch AS run
ENTRYPOINT ["/out/bin/ocicp"]

FROM scratch AS out
COPY --from=build-arm64 /out/bin/ocicp-linux-arm64 /out/bin/ocicp-linux-arm64
COPY --from=build-amd64 /out/bin/ocicp-linux-amd64 /out/bin/ocicp-linux-amd64