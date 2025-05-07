const RPC_URL: &str = "https://api.mainnet-beta.solana.com";
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RpcResponse {
    result: RpcResult,
}

#[derive(Debug, Deserialize)]
struct RpcResult {
    value: u64,
}

pub async fn getting_balance(wallet: &String) -> Result<u64> {
    let client = Client::new();

    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": [wallet]
    });

    let res = client
        .post(RPC_URL)
        .json(&payload)
        .send()
        .await
        .with_context(|| format!("Failed to send request for wallet {}", wallet))?
        .json::<RpcResponse>()
        .await
        .with_context(|| format!("Failed to parse response for wallet {}", wallet))?;

    Ok(res.result.value)
}
