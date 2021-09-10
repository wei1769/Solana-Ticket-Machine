pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};
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