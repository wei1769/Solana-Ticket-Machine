use crate::{
    error::TicketMachineError,
    instruction::TicketSellingPoolInstructions,
    state::{Pool, Ticket},
    check_program_account
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,msg,
    program::invoke,
    program_error::ProgramError,
    program_pack::{ Pack},
    pubkey::Pubkey,
    system_instruction::{self},
    
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = TicketSellingPoolInstructions::unpack(instruction_data)?;
        msg!(&*format!("program id: {}",program_id));
        check_program_account(program_id)?;
        match instruction {
            TicketSellingPoolInstructions::InitPool {
                price,
                fee,
                total_amount,
            } => {
                msg!("Instruction: InitPool");
                Self::process_init_pool(accounts, price, fee, total_amount)
            }
            TicketSellingPoolInstructions::Buy {} => {
                msg!("Instruction: Buy");
                Self::process_buy(accounts)
            }
        }
    }

    fn process_init_pool(
        accounts: &[AccountInfo],
        price: u64,
        fee: u8,
        total: u64,
    ) -> ProgramResult {
        msg!("init pool process");
        let account_info_iter = &mut accounts.iter();
        let pool_id = next_account_info(account_info_iter)?;
        let manager = next_account_info(account_info_iter)?;
        let fee_receiver = next_account_info(account_info_iter)?;
        let mut pool_info = Pool::unpack_unchecked(&pool_id.data.borrow())?;

        if pool_info.account_type != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        if !pool_id.is_signer || !pool_id.is_writable {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !manager.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if fee > 100 {
            msg!("fee is over 100");
            return Err(ProgramError::InvalidAccountData);
        }

        pool_info.account_type = 1;
        pool_info.manager = *manager.key;
        pool_info.fee_reciever = *fee_receiver.key;
        pool_info.total_amount = total;
        pool_info.price = price;
        pool_info.fee = fee;
        pool_info.current_number = 0;
        Pool::pack(pool_info, &mut pool_id.data.borrow_mut())?;

        msg!(&*format!("Pool initialized: {:?}", pool_id.key));
        Ok(())
    }

    fn process_buy(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let pool_id = next_account_info(account_info_iter)?;
        let destination = next_account_info(account_info_iter)?;
        let fee_receiver = next_account_info(account_info_iter)?;
        let ticket_account = next_account_info(account_info_iter)?;
        let buyer = next_account_info(account_info_iter)?;
        let sys_program = next_account_info(account_info_iter)?;
        if !(pool_id.is_writable
            && destination.is_writable
            && fee_receiver.is_writable
            && ticket_account.is_writable)
        {
            msg!("All account should be writable");
            return Err(TicketMachineError::AccountNotWritable.into());
        }

        let mut pool_info = 
            Pool::unpack_unchecked(
                &pool_id.data.borrow())?;


        let mut ticket_info = 
            Ticket::unpack_unchecked(
                &ticket_account.data.borrow())?;

        let current_number = pool_info.current_number.clone();
        if current_number >= pool_info.total_amount.clone(){
            msg!("Pool is Sold out");
           return Err(TicketMachineError::PoolSoldOut.into());
        }
        if !(fee_receiver.key.clone() == pool_info.fee_reciever.clone() &&
            destination.key.clone() == pool_info.manager.clone()){
                return Err(ProgramError::InvalidAccountData);
        }
        if pool_info.account_type.clone() != 1 {
            return Err(ProgramError::InvalidAccountData);
        }

        let fee_amount:u64 = pool_info.price*u64::from(pool_info.fee) / 100  ;

        let pay_amount:u64 = pool_info.price - fee_amount;
        msg!(&*format!("pay: {:?} fee: {:?}",pay_amount,fee_amount));

        let transfer_lamport_ix =
            system_instruction::transfer(
                &buyer.key.clone(),
                 &destination.key.clone(),
                 pay_amount);
        invoke(
            &transfer_lamport_ix,
            &[buyer.clone(),destination.clone(),sys_program.clone()],
        )?;

        let transfer_fee_lamport_ix =
        system_instruction::transfer(
            &buyer.key.clone(),
             &fee_receiver.key.clone(),
             fee_amount);
        invoke(
            &transfer_fee_lamport_ix,
            &[buyer.clone(),fee_receiver.clone(),sys_program.clone()],
        )?;
        let ticket_number = current_number + 1;
        msg!(&*format!("Your Ticket number: {:?}, Pool id: {:?}",ticket_number, pool_id.key));
        ticket_info.account_type = 2;
        ticket_info.pool_id = *pool_id.key;
        ticket_info.ticketbuyer = *buyer.key;
        ticket_info.ticketnumber = ticket_number;
        pool_info.current_number = ticket_number;
        Pool::pack(
            pool_info, 
            &mut pool_id.data.borrow_mut())?;
       
        Ticket::pack(
            ticket_info, 
            &mut ticket_account.data.borrow_mut())?;


        Ok(())
    }
}
