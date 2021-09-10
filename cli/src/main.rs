use crate::util::get_pub;
mod ticket_finding;
mod util;
use std::{env};
use solana_client::{rpc_client::RpcClient};
use solana_sdk::{ commitment_config::CommitmentConfig};


fn main() {
    let arg: Vec<String> = env::args().collect();
    let pool_id = get_pub(&arg[1]);
    let rpc_url: String = "https://api.devnet.solana.com".to_string();
    //change this URL to change Cluster
    let commitment = CommitmentConfig::confirmed();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, commitment);

    let tickets = ticket_finding::findtickets(&pool_id,rpc_client);

    for data in tickets {
        println!("{:?},{:?}", data.0, data.1);
    }
}
