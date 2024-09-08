use alloy::providers::{Provider, ProviderBuilder};
use dotenv::dotenv;
use eyre::Result;
use std::env;

fn get_api_key() -> String {
    // Load .env file if present
    dotenv().ok();
    env::var("RPC_URL").expect("RPC_URL must be set")
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = get_api_key().parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let latest_block_number = provider.get_block_number().await?;
    println!("Latest block number: {latest_block_number}");

    Ok(())
}
