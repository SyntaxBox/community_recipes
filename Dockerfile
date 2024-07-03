ARG RUST_VERSION=latest

FROM rust:${RUST_VERSION} AS builder
WORKDIR /app
COPY . .
RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --release && \
  cp ./target/release/community_recipe /


FROM debian:bookworm-slim AS final

COPY --from=builder /community_recipe /usr/local/bin

ENV RUST_LOG="community_recipe=debug,info"
CMD ["community_recipe"]