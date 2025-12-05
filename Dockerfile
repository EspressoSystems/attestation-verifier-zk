FROM rustlang/rust:nightly-bookworm as builder

ARG BIN_NAME=attestation-verifier-zk
WORKDIR /app

COPY . .

RUN cargo build --release --bin ${BIN_NAME}

FROM debian:bookworm-slim as runtime

ARG BIN_NAME=attestation-verifier-zk
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m appuser

WORKDIR /app
COPY --from=builder /app/target/release/${BIN_NAME} /usr/local/bin/app


EXPOSE 8080

USER appuser
# Start the service
CMD ["app"]