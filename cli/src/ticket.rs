use std::borrow::Borrow;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{rpc_client::RpcClient,rpc_config::{RpcProgramAccountsConfig,RpcAccountInfoConfig},rpc_filter::{RpcFilterType,Memcmp,MemcmpEncodedBytes}};
use solana_program::{program_pack::{ Pack}};




use solana_sdk::{ commitment_config::CommitmentConfig, instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::{Keypair, Signer}, system_instruction::create_account,};

use crate::util::{get_pub,Pool,Ticket};



pub fn init_pool(
    manger: &Pubkey,
    fee_receiver: &Pubkey,
    price: u64,
    fee: u8,
    total_amount: u64,
    connection: RpcClient,
) -> (Vec<Instruction>, Keypair ) {
    let ticket_program_id = get_pub("AUaGuQhpjttMdBmejoboMoUMrpcxNHZsT44C6jupLYNP");
    let mut ins: Vec<Instruction> = vec![];
    let pool_key = Keypair::new();
    let lamport_needed = connection
        .get_minimum_balance_for_rent_exemption(95)
        .unwrap();
    let create_account_ins = create_account(
        manger,
        &pool_key.pubkey(),
        lamport_needed,
        90,
        &ticket_program_id,
    );
    ins.push(create_account_ins);
    let mut data: Vec<u8> = vec![];
    let mut keys: Vec<AccountMeta> = vec![];
    keys.push(AccountMeta::new(pool_key.pubkey(), true));
    keys.push(AccountMeta::new(*manger.borrow(), true));
    keys.push(AccountMeta::new(*fee_receiver.borrow(), false));

    data.push(0);
    data.extend_from_slice(&price.to_le_bytes());
    data.extend_from_slice(&fee.to_le_bytes());
    data.extend_from_slice(&total_amount.to_le_bytes());
    let init_pool_ins = Instruction {
        program_id: ticket_program_id,
        data: data,
        accounts: keys,
    };
    ins.push(init_pool_ins);

    (ins, pool_key)
}
pub fn buy(pool_id:&Pubkey,buyer:&Pubkey,connection: RpcClient,)-> (Vec<Instruction>, Keypair){
    let ticket_program_id = get_pub("AUaGuQhpjttMdBmejoboMoUMrpcxNHZsT44C6jupLYNP");
    let mut ins: Vec<Instruction> = vec![];
    let ticket_key = Keypair::new();
    let lamport_needed = connection
        .get_minimum_balance_for_rent_exemption(73)
        .unwrap();
    let create_account_ins = create_account(
        buyer,
        &ticket_key.pubkey(),
        lamport_needed,
        73,
        &ticket_program_id,
    );
    ins.push(create_account_ins);

    let poo_info = self::find_pool(pool_id);
    let mut data: Vec<u8> = vec![];
    let mut keys: Vec<AccountMeta> = vec![];
    keys.push(AccountMeta::new(pool_id.clone(),false));
    keys.push(AccountMeta::new(poo_info.manager.clone(),false));
    keys.push(AccountMeta::new(poo_info.fee_reciever.clone(),false));
    keys.push(AccountMeta::new(ticket_key.pubkey().clone(),true));
    keys.push(AccountMeta::new(buyer.clone(),true));
    keys.push(AccountMeta::new_readonly(get_pub("11111111111111111111111111111111"), false));
    data.push(1);
    let buy_ins = Instruction{
        program_id:ticket_program_id.clone(),
        data:data,
        accounts:keys,
    };
    ins.push(buy_ins);


    (ins, ticket_key)
}


pub fn find_pool(pool_id:&Pubkey) -> Pool{
    let rpc_url: String = "https://api.devnet.solana.com".to_string();
    let commitment = CommitmentConfig::confirmed();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, commitment);
    
    let pool_info =rpc_client.get_account_data(pool_id).unwrap();
    let poo_info_ser = Pool::unpack_unchecked(&pool_info).unwrap();
    poo_info_ser

}

pub fn findtickets(pool_id:&Pubkey, connection: RpcClient)-> Vec<(u64,Pubkey)>{
    let mut ticket_data:Vec<(u64,Pubkey)> = vec!();
    let ticket_program_id = get_pub("AUaGuQhpjttMdBmejoboMoUMrpcxNHZsT44C6jupLYNP");

    


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

    let  accounts = connection.get_program_accounts_with_config(&ticket_program_id,config).unwrap();
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

