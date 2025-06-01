use clap::Parser;
use nft_verifier_lib::NFTProofPublicValues;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::str::FromStr;
use ethers::prelude::*;
use std::sync::Arc;

/// Path ke program ELF VM
pub const NFT_VERIFIER_ELF: &[u8] = include_elf!("nft-verifier-program");

/// Command line arguments
#[derive(Parser, Debug)]
struct Args {
    /// Wallet address user (hex, boleh pakai/enggak pakai 0x, besar/kecil)
    #[arg(long)]
    wallet: String,
    /// Alamat contract NFT (hex)
    #[arg(long)]
    ca: String,
    /// Token ID NFT (u128)
    #[arg(long, value_name = "TOKEN_ID")]
    token_id: u128,
    /// Eksekusi biasa (tanpa proof)
    #[arg(long)]
    execute: bool,
    /// Jalankan mode proof (SP1/zkVM)
    #[arg(long)]
    prove: bool,
    /// RPC URL (optional, default ke Berachain public)
    #[arg(long, default_value = "https://rpc.berachain.com")]
    rpc_url: String,
}

/// Fungsi parsing address: hilangkan spasi, awalan 0x, case-insensitive, 20 byte array
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

// Minimal ABI ownerOf
abigen!(
    IERC721,
    r#"[
        function ownerOf(uint256 tokenId) view returns (address)
    ]"#
);

#[tokio::main]
async fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();
    let args = Args::parse();

    // Validasi mode run
    if args.execute == args.prove {
        eprintln!("Error: Pilih salah satu --execute atau --prove!");
        std::process::exit(1);
    }

    // Inisialisasi provider ethers
    let provider = match Provider::<Http>::try_from(args.rpc_url.as_str()) {
        Ok(p) => Arc::new(p),
        Err(e) => {
            eprintln!("Gagal konek ke RPC: {e}");
            std::process::exit(1);
        }
    };

    // Siapkan kontrak NFT
    let ca_addr = Address::from_str(&args.ca).expect("Invalid contract address");
    let nft = IERC721::new(ca_addr, provider.clone());

    // Ambil owner onchain (fix: konversi token_id ke U256)
    let owner_addr = match nft.owner_of(args.token_id.into()).call().await {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Gagal mengambil owner onchain: {e}");
            std::process::exit(1);
        }
    };

    // Parsing semua address ke [u8; 20]
    let wallet_bytes = parse_addr(&args.wallet);
    let ca_bytes = parse_addr(&args.ca);
    let owner_bytes = owner_addr.0; // ethers-rs Address inner [u8; 20]

    // Debug print sebelum masuk ke VM
    println!("=== Debug Info ===");
    println!("wallet arg:    '{}'", args.wallet);
    println!("owner onchain: '0x{}'", hex::encode(owner_bytes));
    println!("wallet_bytes:  {:?}", wallet_bytes);
    println!("owner_bytes:   {:?}", owner_bytes);
    println!("Equal bytes?   {}\n", wallet_bytes == owner_bytes);

    // Siapkan input untuk VM
    let client = ProverClient::from_env();
    let mut stdin = SP1Stdin::new();
    // Urutan harus sama dengan urutan di ELF/VM
    stdin.write(&(wallet_bytes, ca_bytes, args.token_id, owner_bytes));

    if args.execute {
        let (output, _) = client.execute(NFT_VERIFIER_ELF, &stdin).run().unwrap();
        let decoded: NFTProofPublicValues = bincode::deserialize(output.as_slice()).unwrap();
        println!("wallet:   0x{}", hex::encode(decoded.wallet));
        println!("ca:       0x{}", hex::encode(decoded.ca));
        println!("token_id: {}", decoded.token_id);
        println!(
            "{}",
            if decoded.has_nft {
                "Congrats you're owner of Steady teddys NFT. thank you for being TEDISM right now"
            } else {
                "Sorry you dont have a steady teddys NFT. please buy it on secondary marketplace like Magic eden. sorry for ur loss"
            }
        );
    } else {
        // Mode proof
        let (pk, vk) = client.setup(NFT_VERIFIER_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("Gagal membuat proof");

        println!("Berhasil membuat proof!");

        // Verify proof
        client.verify(&proof, &vk).expect("Proof gagal diverifikasi");
        println!("Proof berhasil diverifikasi!");

        // Decode dan tampilkan output
        let decoded: NFTProofPublicValues = bincode::deserialize(proof.public_values.as_slice()).unwrap();
        println!("wallet:   0x{}", hex::encode(decoded.wallet));
        println!("ca:       0x{}", hex::encode(decoded.ca));
        println!("token_id: {}", decoded.token_id);
        println!(
            "{}",
            if decoded.has_nft {
                "anda mempunyai NFT ini"
            } else {
                "anda tidak mempunyai NFT ini"
            }
        );
    }
}
