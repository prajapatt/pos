# syntax=docker/dockerfile:1

FROM rust:1.82.0-bookworm AS builder
WORKDIR /src

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo test --lib --quiet && cargo build --release

FROM debian:12.7-slim AS runtime
ENV DEBIAN_FRONTEND=noninteractive
WORKDIR /release

RUN apt-get update && apt-get upgrade -y && apt-get install -y --no-install-recommends bash ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /src/Cargo.toml /release/
COPY --from=builder /src/rust-toolchain.toml /release/
COPY --from=builder /src/README.md /release/
COPY --from=builder /src/target/release /release/bin/

RUN mkdir -p /release/artifacts && find /release/bin -maxdepth 1 -type f -exec cp {} /release/artifacts/ \; 2>/dev/null || true

CMD ["bash", "-lc", "ls -la /release && echo 'POS release bundle is available in /release/artifacts'"]
