#![no_main]
sp1_zkvm::entrypoint!(main);

use sp1_zkvm::io;
use nft_verifier_lib::NFTProofPublicValues;

pub fn main() {
    let (wallet, ca, token_id, owner): ([u8; 20], [u8; 20], u128, [u8; 20]) = io::read();
    let has_nft = wallet == owner;
    let pub_values = NFTProofPublicValues {
        wallet,
        ca,
        token_id,
        has_nft,
    };
    io::commit_slice(&bincode::serialize(&pub_values).unwrap());
}
