use alloy::{
    eips::BlockNumberOrTag,
    providers::{Provider, ProviderBuilder},
};
use alloy_rpc_types_eth::Block;
use dotenv::dotenv;
use eyre::Result;
use std::env;

fn get_api_key() -> String {
    // Load .env file if present
    // https://eth.merkle.io is free
    dotenv().ok();
    env::var("RPC_URL").expect("RPC_URL must be set")
}

fn handle_block(block: Block) {
    let transactions = block.transactions;

    if let Some(max_gas_tx) = transactions
        .as_transactions()
        .unwrap_or(&[])
        .iter()
        .max_by_key(|tx| tx.max_fee_per_gas)
    {
        if let Some(max_fee) = max_gas_tx.max_fee_per_gas {
            println!(
                "Latest block number: {}, Tx with highest base fee gas: {:?}",
                block.header.number, max_fee
            );
        } else {
            println!("Transaction has not max_fee_per_gas field");
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = get_api_key().parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let latest_block = provider
        .get_block_by_number(BlockNumberOrTag::Latest, true)
        .await?;
    if let Some(block) = latest_block {
        handle_block(block);
    } else {
        println!("Unable to get latest block");
    }

    Ok(())
}
