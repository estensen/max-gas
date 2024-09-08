use alloy::{
    eips::BlockNumberOrTag,
    providers::{Provider, ProviderBuilder},
};
use alloy_rpc_types_eth::{Block, BlockTransactions, Transaction};
use dotenv::dotenv;
use eyre::Result;
use std::env;

fn get_api_key() -> String {
    // Load .env file if present
    // https://eth.merkle.io is free
    dotenv().ok();
    env::var("RPC_URL").expect("RPC_URL must be set")
}

fn get_max_fee(transactions: &BlockTransactions<Transaction>) -> Option<&Transaction> {
    transactions
        .as_transactions()
        .unwrap_or(&[])
        .iter()
        .max_by_key(|tx| tx.max_fee_per_gas)
}

fn handle_block(block: Block) {
    let transactions = block.transactions;

    let max_gas_tx = get_max_fee(&transactions);
    if let Some(max_fee) = max_gas_tx.unwrap().max_fee_per_gas {
        println!(
            "Latest block number: {}, Tx with highest base fee gas: {:?}",
            block.header.number, max_fee
        );
    } else {
        println!("Transaction has not max_fee_per_gas field");
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_transaction(max_fee_per_gas: Option<u128>) -> Transaction {
        Transaction {
            max_fee_per_gas: max_fee_per_gas,
            ..Default::default()
        }
    }

    #[test]
    fn test_get_max_fee() {
        let test_cases = vec![
            ("No transactions", BlockTransactions::Full(vec![]), None),
            (
                "Single transaction with max_fee_per_gas",
                BlockTransactions::Full(vec![create_transaction(Some(100))]),
                Some(100),
            ),
            (
                "Multiple transactions, first has highest max_fee_per_gas",
                BlockTransactions::Full(vec![
                    create_transaction(Some(200)),
                    create_transaction(Some(100)),
                ]),
                Some(200),
            ),
            (
                "Multiple transactions, second has highest max_fee_per_gas",
                BlockTransactions::Full(vec![
                    create_transaction(Some(100)),
                    create_transaction(Some(300)),
                ]),
                Some(300),
            ),
            (
                "Transactions without max_fee_per_gas",
                BlockTransactions::Full(vec![create_transaction(None), create_transaction(None)]),
                None,
            ),
        ];

        for (description, transactions, expected) in test_cases {
            let result = get_max_fee(&transactions).and_then(|tx| tx.max_fee_per_gas);

            assert_eq!(result, expected, "Test failed for case: {}", description);
        }
    }
}
