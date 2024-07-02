ARG RUST_VERSION=latest

FROM rust:${RUST_VERSION} AS builder
WORKDIR /app
COPY . .
RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --release && \
  cp ./target/release/salamtak_server /


FROM debian:bookworm-slim AS final

COPY --from=builder /salamtak_server /usr/local/bin

ENV RUST_LOG="salamtak_server=debug,info"
CMD ["salamtak_server"]