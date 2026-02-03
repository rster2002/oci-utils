FROM rust:1 AS base

WORKDIR /project
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src ./src

RUN mkdir -p /out/bin

FROM base AS build-arm64
RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo build \
      --release \
      --target=aarch64-unknown-linux-gnu

RUN cp ./target/aarch64-unknown-linux-gnu/release/ocicp /out/bin/ocicp

FROM base AS build-amd64
RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build \
      --release \
      --target=x86_64-unknown-linux-gnu

RUN cp ./target/x86_64-unknown-linux-gnu/release/ocicp /out/bin/ocicp

ARG TARGETARCH
FROM build-$TARGETARCH AS final

FROM scratch AS out
COPY --from=final /out/bin /out/bin

ENTRYPOINT ["/out/bin/ocicp"]