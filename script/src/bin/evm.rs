//! Contoh end-to-end menggunakan SP1 SDK untuk generate EVM-compatible proof (Groth16/PLONK) yang bisa diverifikasi onchain di Solidity.
//!
//! Jalankan dengan:
//! ```shell
//! RUST_LOG=info cargo run --release --bin evm -- --system groth16
//! # atau
//! RUST_LOG=info cargo run --release --bin evm -- --system plonk
//! ```

use clap::{Parser, ValueEnum};
use nft_verifier_lib::NFTProofPublicValues;
use serde::{Deserialize, Serialize};
use sp1_sdk::{
    include_elf, HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey,
};
use std::path::PathBuf;

/// Ganti dengan ELF zkVM program NFT-mu!
pub const NFT_VERIFIER_ELF: &[u8] = include_elf!("nft-verifier-program");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct EVMArgs {
    /// Wallet address (hex 20 byte)
    #[arg(long)]
    wallet: String,
    /// Contract address (hex 20 byte)
    #[arg(long)]
    ca: String,
    /// Token ID (u128)
    #[arg(long)]
    token_id: u128,
    /// Owner address (hex 20 byte, hasil onchain)
    #[arg(long)]
    owner: String,
    /// Proof system
    #[arg(long, value_enum, default_value = "groth16")]
    system: ProofSystem,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ProofSystem {
    Plonk,
    Groth16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1NFTProofFixture {
    wallet: String,
    ca: String,
    token_id: u128,
    owner: String,
    vkey: String,
    public_values: String,
    proof: String,
}

fn parse_addr(s: &str) -> [u8; 20] {
    let clean = s.trim()
        .trim_start_matches("0x")
        .to_ascii_lowercase();
    let bytes = hex::decode(clean).expect("Invalid hex address");
    assert_eq!(bytes.len(), 20, "Address must be 20 bytes (40 hex digits)");
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    arr
}

fn main() {
    sp1_sdk::utils::setup_logger();
    let args = EVMArgs::parse();
    let client = ProverClient::from_env();

    // Setup the program.
    let (pk, vk) = client.setup(NFT_VERIFIER_ELF);

    // Setup the inputs.
    let wallet_bytes = parse_addr(&args.wallet);
    let ca_bytes = parse_addr(&args.ca);
    let owner_bytes = parse_addr(&args.owner);
    let mut stdin = SP1Stdin::new();
    stdin.write(&(wallet_bytes, ca_bytes, args.token_id, owner_bytes));

    println!("wallet: {}", args.wallet);
    println!("ca:     {}", args.ca);
    println!("token_id: {}", args.token_id);
    println!("owner:  {}", args.owner);
    println!("Proof System: {:?}", args.system);

    // Generate the proof based on the selected proof system.
    let proof = match args.system {
        ProofSystem::Plonk => client.prove(&pk, &stdin).plonk().run(),
        ProofSystem::Groth16 => client.prove(&pk, &stdin).groth16().run(),
    }
    .expect("failed to generate proof");

    create_proof_fixture(&proof, &vk, &args);
}

fn create_proof_fixture(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
    args: &EVMArgs,
) {
    // Deserialize the public values.
    let bytes = proof.public_values.as_slice();
    let public: NFTProofPublicValues = bincode::deserialize(bytes).unwrap();

    let fixture = SP1NFTProofFixture {
        wallet: format!("0x{}", hex::encode(public.wallet)),
        ca: format!("0x{}", hex::encode(public.ca)),
        token_id: public.token_id,
        owner: args.owner.clone(),
        vkey: vk.bytes32().to_string(),
        public_values: format!("0x{}", hex::encode(bytes)),
        proof: format!("0x{}", hex::encode(proof.bytes())),
    };

    println!("Verification Key: {}", fixture.vkey);
    println!("Public Values: {}", fixture.public_values);
    println!("Proof Bytes: {}", fixture.proof);

    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../contracts/src/fixtures");
    std::fs::create_dir_all(&fixture_path).expect("failed to create fixture path");
    std::fs::write(
        fixture_path.join(format!("{:?}-fixture.json", args.system).to_lowercase()),
        serde_json::to_string_pretty(&fixture).unwrap(),
    ).expect("failed to write fixture");
}
