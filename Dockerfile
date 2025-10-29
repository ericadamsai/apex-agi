# Dockerfile for Apex AGI - ericadamsai watermark
# Multi-stage build for optimized production image

FROM rust:latest as builder
WORKDIR /usr/src/apex-agi
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/apex-agi/target/release/apex_server /app/apex_server
COPY --from=builder /usr/src/apex-agi/target/release/apex_cli /app/apex_cli

# ericadamsai - AGI System Container
LABEL maintainer="ericadamsai"
LABEL version="0.1.0"
LABEL description="Apex AGI System Container - ericadamsai watermark"

EXPOSE 8080
ENV RUST_LOG=info
CMD ["./apex_server"]
