use anyhow::{bail, Result};
use solana_sdk::signature::Keypair;

pub fn parse_keypair_base58(s: &str) -> Result<Keypair> {
    let bytes = bs58::decode(s).into_vec()?;
    if bytes.len() != 64 {
        bail!("Expected 64-byte keypair, got {}", bytes.len());
    }
    Ok(Keypair::from_bytes(&bytes)?)
}
