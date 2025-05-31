use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NFTProofPublicValues {
    pub wallet: [u8; 20],
    pub ca: [u8; 20],
    pub token_id: u128,
    pub has_nft: bool,
}

pub fn verify_nft_ownership(wallet: [u8; 20], ca: [u8; 20], token_id: u128) -> bool {
    let owner: [u8; 20] = evm::owner_of(ca, token_id);
    owner == wallet
}

mod evm {
    pub fn owner_of(_ca: [u8; 20], _token_id: u128) -> [u8; 20] {
        [0u8; 20]
    }
}
