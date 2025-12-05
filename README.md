# Attestation Verifier ZK

A web service that generates zero-knowledge proofs for AWS Nitro Enclave attestation reports using SP1.

## Setup

1. Copy `.env.example` to `.env` and configure:

```env
NITRO_VERIFIER_ADDRESS=<your-verifier-contract-address>
RPC_URL=<your-ethereum-rpc-url>
SUCCINCT_PRIVATE_KEY=<your-succinct-private-key>
SUCCINCT_NETWORK_RPC_URL=<succinct-network-rpc-url>
SP1_PROVER=network  # or "mock" for testing
RUST_LOG=info
```

2. Run the server:

```bash
cargo run --release
```

## Usage

Generate a proof by sending an attestation report to the API:

```bash
curl -X POST http://127.0.0.1:8080/generate_proof \
  --data-binary @sample_reports/nitro_attestation_data.bin
```

## Development

```bash
# Run tests
cargo test

# Check formatting and linting
cargo fmt --all -- --check
cargo clippy --all-targets --all-features
```

## Docker

To run using docker simply create a .env file and run `docker compose up`
