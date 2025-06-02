# sp1-nft-verifier

**sp1-nft-verifier** is a Rust-based NFT verification project for the SP1 ecosystem. The project is organized as a modular Rust workspace, focused on NFT verification logic and designed to run with the SP1 proof system in various modes.

---

## Features

- NFT verification logic for EVM-compatible chains, written in Rust for SP1
- Modular Rust workspace structure (`lib`, `program`, `script`)
- Easily configurable SP1 prover modes (`mock`, `cpu`, `cuda`, `network`)
- Flexible CLI for execution or proving (see Usage)
- Example environment configuration for fast setup
- GitHub Actions workflow for continuous integration

---

## Requirements

Before you begin, ensure you have installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Go](https://go.dev/doc/install) (required by [SP1](https://github.com/succinctlabs/sp1))
- [SP1](https://github.com/succinctlabs/sp1) — follow the official SP1 docs for installation and dependencies  
  - For EVM-specific proof/execution support, see [SP1 EVM Documentation](https://github.com/succinctlabs/sp1/blob/main/docs/evm.md) or the [SP1 docs site](https://docs.succinct.xyz/docs/sp1/evm/overview)

---

## Directory Structure

```
.
├── Cargo.toml         # Rust workspace configuration
├── Cargo.lock         # Cargo lock file
├── LICENSE-MIT        # MIT License
├── README.md          # Project documentation
├── rust-toolchain     # Rust toolchain version specification
├── .env.example       # Example environment variable file
├── lib/               # Core NFT verifier logic
├── program/           # Main application entry point
├── script/            # Additional Rust binaries or build scripts
```

---

## Environment Configuration

Copy the example environment file and update as needed:
```bash
cp .env.example .env
```

Example `.env` file:
```
# Proof modes are `mock`, `cpu`, `cuda` and `network`.
SP1_PROVER=cpu

# To use the Succinct Prover Network, set the private key of the account you want to use for requesting proofs.
# NETWORK_PRIVATE_KEY=your_network_private_key
```

- `SP1_PROVER`: Selects the proof mode. Options: `mock`, `cpu`, `cuda`, `network`.
- `NETWORK_PRIVATE_KEY`: Only needed when `SP1_PROVER=network`. Wallet selection is automatic; do not set or reference `pk` elsewhere.

---

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/taufnashrul04/sp1-nft-verifier.git
cd sp1-nft-verifier
```

### 2. Install dependencies

- [Install Rust](https://www.rust-lang.org/tools/install)
- [Install Go](https://go.dev/doc/install)
- [Install SP1](https://github.com/succinctlabs/sp1) (see SP1 docs for details and required dependencies)

### 3. Build the project

```bash
cargo build --release --workspace
```

### 4. Prepare the environment

Copy and edit your `.env` as described above.

---

## Usage

## Verification nft
1. this script is for use to verify steady teddys nft
2. if you need to verify another nft in berachain you can use it
3. if you use to verify another nft in another evm chain you can change RPC at main.rs

### Run Modes

The verifier supports both "execute" (run EVM logic and check NFT) and "prove" (generate SP1 proof for the NFT verification).  
You can run either mode using CLI arguments.

### Example: Verify an NFT (execute or prove)

```bash
cargo run --release --bin main -- \
  --execute \
  --wallet 0x1234567890abcdef1234567890abcdef12345678 \
  --token-id 479
```

or, to generate a proof:

```bash
cargo run --release --bin main -- \
  --prove \
  --wallet 0x1234567890abcdef1234567890abcdef12345678 \
  --token-id 479
```
#notes
this command above is for verify steady teddy nft
if you want a verify another nft use this command
```bash
cargo run --release --bin main -- \
  --execute \ or prove
  --wallet 0x1234567890abcdef1234567890abcdef12345678 \
  --ca 0xb40ba1951b9b3be813f5a4ceafed29eb08d5358d \
  --token-id 479
  --rpc-url https://rpc-public-of-your-nft-chain
```
#notes
**Parameters:**
- `--execute` : Run the EVM logic and check the NFT (no proof)
- `--prove`   : Generate a proof using the configured `SP1_PROVER` backend
- `--wallet`  : EVM wallet address for the NFT owner (example: `0x1234567890abcdef1234567890abcdef12345678`)
- `--ca`      : NFT contract address
- `--token-id`: Token ID of the NFT to verify
- `--rpc-url` : RPc of your nft chain place
- `RUST_LOG=info`: Before cargo run to show log

> For details on EVM options and integration, see the [SP1 EVM Documentation](https://docs.succinct.xyz/docs/sp1/evm/overview).

---

## Continuous Integration

This repository includes a GitHub Actions workflow for Rust:

- **Formatting Check** (`cargo fmt`)
- **Linting** (`cargo clippy`)
- **Build** (`cargo build --workspace`)
- **Test** (`cargo test --workspace`)
- **Artifacts Upload** (optional)
- **Rust cache** for faster builds

See `.github/workflows/rust.yml` for details.

---

## Contributing

Pull requests, issues, and discussions are welcome! Please open an issue to discuss your ideas or bug reports, especially those related to Rust, EVM, or SP1 integration.

---

## License

This project is licensed under the MIT License. See [LICENSE-MIT](LICENSE-MIT) for details.
