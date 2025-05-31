use sp1_sdk::{include_elf, ProverClient, Prover, HashableKey};

/// ELF file for SP1 zkVM program (NFT verifier)
pub const NFT_VERIFIER_ELF: &[u8] = include_elf!("nft-verifier-program");

fn main() {
    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(NFT_VERIFIER_ELF);
    println!("{}", vk.bytes32());
}
