use clap::Parser;
use nft_verifier_lib::NFTProofPublicValues;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::str::FromStr;
use ethers::prelude::*;
use std::sync::Arc;

/// Path ke program ELF VM
pub const NFT_VERIFIER_ELF: &[u8] = include_elf!("nft-verifier-program");

/// Default Steady Teddy CA & RPC
const DEFAULT_CA: &str = "0x88888888a9361f15aadbaca355a6b2938c6a674e";
const DEFAULT_RPC: &str = "https://rpc.berachain.com";

/// Command line arguments
#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    wallet: String,
    #[arg(long, default_value = DEFAULT_CA)]
    ca: String,
    #[arg(long, value_name = "TOKEN_ID")]
    token_id: u128,
    #[arg(long)]
    execute: bool,
    #[arg(long)]
    prove: bool,
    #[arg(long, default_value = DEFAULT_RPC)]
    rpc_url: String,
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

// Tambahkan fungsi name() ke abigen
abigen!(
    IERC721,
    r#"[
        function ownerOf(uint256 tokenId) view returns (address)
        function name() view returns (string)
    ]"#
);

#[tokio::main]
async fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: choose --execute or --prove!");
        std::process::exit(1);
    }

    let provider = match Provider::<Http>::try_from(args.rpc_url.as_str()) {
        Ok(p) => Arc::new(p),
        Err(e) => {
            eprintln!("can't connect to RPC: {e}");
            std::process::exit(1);
        }
    };

    let ca_addr = Address::from_str(&args.ca).expect("Invalid contract address");
    let nft = IERC721::new(ca_addr, provider.clone());

    // --- Ambil nama koleksi NFT ---
    let nft_name = match nft.name().call().await {
        Ok(name) => name,
        Err(_) => "(can't get name NFT)".to_string(),
    };

    // --- Ambil owner ---
    let owner_addr = match nft.owner_of(args.token_id.into()).call().await {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("can't get owner onchain: {e}");
            std::process::exit(1);
        }
    };

    let wallet_bytes = parse_addr(&args.wallet);
    let ca_bytes = parse_addr(&args.ca);
    let owner_bytes = owner_addr.0;

    println!("=== Debug Info ===");
    println!("NFT name:     '{}'", nft_name);
    println!("wallet arg:   '{}'", args.wallet);
    println!("owner chain:  '0x{}'", hex::encode(owner_bytes));
    println!("Equal bytes?  {}\n", wallet_bytes == owner_bytes);

    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();
    stdin.write(&(wallet_bytes, ca_bytes, args.token_id, owner_bytes));

    if args.execute {
        let (output, _) = client.execute(NFT_VERIFIER_ELF, &stdin).run().unwrap();
        let decoded: NFTProofPublicValues = bincode::deserialize(output.as_slice()).unwrap();
        println!("NFT name:   '{}'", nft_name);
        println!("wallet:     0x{}", hex::encode(decoded.wallet));
        println!("ca:         0x{}", hex::encode(decoded.ca));
        println!("token_id:   {}", decoded.token_id);
        println!(
            "{}",
            if decoded.has_nft {
                format!("Congrats, you're owner of {} NFT. Thank you for use SP1 right now", nft_name)
            } else {
                format!("Sorry, you don't have a {} NFT. Please buy it on secondary marketplace. Thank you for use SP1 right now", nft_name)
            }
        );
    } else {
        let (pk, vk) = client.setup(NFT_VERIFIER_ELF);
        let proof = client.prove(&pk, &stdin).run().expect("Gagal membuat proof");

        println!("successfully create proof!");
        client.verify(&proof, &vk).expect("can't verify proof");
        println!("Proof verify successful!");

        let decoded: NFTProofPublicValues = bincode::deserialize(proof.public_values.as_slice()).unwrap();
        println!("NFT name:   '{}'", nft_name);
        println!("wallet:     0x{}", hex::encode(decoded.wallet));
        println!("ca:         0x{}", hex::encode(decoded.ca));
        println!("token_id:   {}", decoded.token_id);
        println!(
            "{}",
            if decoded.has_nft {
                format!("Congrats, you're owner of {} NFT. Thank you for use SP1 right now", nft_name)
            } else {
                format!("Sorry, you don't have a {} NFT. Please buy it on secondary marketplace. Thank you for use SP1 right now", nft_name)
            }
        );
    }
}
