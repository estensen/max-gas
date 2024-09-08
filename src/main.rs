use dotenv::dotenv;
use std::env;

fn get_api_key() -> String {
    // Load .env file if present
    dotenv().ok();
    env::var("RPC_URL").expect("RPC_URL must be set")
}

fn main() {
    let rpc_url = get_api_key();
    println!("RPC URL: {}", rpc_url);
}
