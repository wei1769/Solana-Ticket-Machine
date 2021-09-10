
use solana_account_decoder::UiAccountEncoding;


use solana_client::{rpc_client::RpcClient,rpc_config::{RpcProgramAccountsConfig,RpcAccountInfoConfig},rpc_filter::{RpcFilterType,Memcmp,MemcmpEncodedBytes}};
use solana_program::{  program_pack::{ Pack, }};




use solana_sdk::{ commitment_config::CommitmentConfig,   pubkey::Pubkey,  };

use crate::util::{get_pub,Ticket};





pub fn findtickets(pool_id:&Pubkey)-> Vec<(u64,Pubkey)>{
    let mut ticket_data:Vec<(u64,Pubkey)> = vec!();
    let ticket_program_id = get_pub("AUaGuQhpjttMdBmejoboMoUMrpcxNHZsT44C6jupLYNP");

    let rpc_url: String = "https://api.devnet.solana.com".to_string();
    let commitment = CommitmentConfig::confirmed();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, commitment);
    


    let mut mem:Vec<u8> = vec![2];
    mem.extend_from_slice(&pool_id.to_bytes());
    let memcmp =  MemcmpEncodedBytes::Binary(bs58::encode(mem).into_string());
    //println!("memcmp: {:?}\n",memcmp);
    let filter = Some(vec![ 
        RpcFilterType::Memcmp(
            Memcmp{
                offset: 0,bytes:memcmp,encoding:None})
        ]);
    
    let config = RpcProgramAccountsConfig{filters: filter,account_config:RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64),
        ..RpcAccountInfoConfig::default()},
        with_context:None
    };

    let  accounts = rpc_client.get_program_accounts_with_config(&ticket_program_id,config).unwrap();
    //println!("{:?}",accounts);
    for data in accounts {
        let account = data.1;
        let current_ticket = Ticket::unpack_unchecked(&account.data).unwrap();
        let ticket_number = current_ticket.ticketnumber;
        let ticket_buyer = current_ticket.ticketbuyer;
        ticket_data.push((ticket_number,ticket_buyer));
        
    }
    ticket_data.sort();
    
    ticket_data
}

