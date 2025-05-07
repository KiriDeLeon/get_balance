use keypair::get_keypair::parse_keypair_base58;
use anyhow::Result;
use balance::get_balance::getting_balance;
use fs::read_yaml::load_config;
use futures::future::join_all;
use solana_sdk::signer::Signer;

pub mod keypair;
pub mod balance;
pub mod fs;

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config("data/wallet.yaml")?;

    let tasks = config.wallets.iter().map(|encoded| async move {
        match parse_keypair_base58(encoded) {
            Ok(keypair) => {
                let pubkey = keypair.pubkey().to_string();
                match getting_balance(&pubkey).await {
                    Ok(balance) => println!("Wallet: {} balance: {}", pubkey, balance),
                    Err(e) => eprintln!("Error getting balance for {}: {:?}", pubkey, e),
                }
            }
            Err(e) => eprintln!("Invalid keypair: {:?} source: {}", e, encoded),
        }
    });

    join_all(tasks).await;

    Ok(())
}
