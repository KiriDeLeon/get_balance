use anyhow::Result;
use balance::get_balance::getting_balance;
use fs::read_yaml::load_config;
use futures::future::join_all;

pub mod balance;
pub mod fs;

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config("data/wallet.yaml")?;

    let tasks = config.wallets.iter().map(|wallet| async move {
        match getting_balance(wallet).await {
            Ok(balance) => println!("Wallet: {} balance: {} ", wallet, balance),
            Err(err) => eprintln!("Error for {}: {:?}", wallet, err),
        }
    });

    join_all(tasks).await;

    Ok(())
}
