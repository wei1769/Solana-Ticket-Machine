pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
use std::str::FromStr;

use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey,msg};
#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

solana_program::declare_id!("AUaGuQhpjttMdBmejoboMoUMrpcxNHZsT44C6jupLYNP");
/// Checks that the supplied program ID is the correct one for SPL-token
pub fn check_program_account(spl_token_program_id: &Pubkey) -> ProgramResult {
    if spl_token_program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
pub fn check_fee_account(fee_reciever_id: &Pubkey) -> ProgramResult{
    if fee_reciever_id.clone() != Pubkey::from_str("2wnEcArzCpX1QRdtpHRXxZ7k9b1UeK16mPt26LPWFZ6V").unwrap(){
        msg!("Fee reciever is wrong");
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}
