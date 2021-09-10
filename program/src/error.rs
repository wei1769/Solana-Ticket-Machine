use thiserror::Error;
use solana_program::program_error::ProgramError;
#[derive(Error, Debug, Copy, Clone)]
pub enum TicketMachineError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    ///Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Account Not Writable")]
    AccountNotWritable,
    #[error("Ticket has been sold out")]
    PoolSoldOut
}

impl From<TicketMachineError> for ProgramError {
    fn from(e: TicketMachineError) -> Self {
        ProgramError::Custom(e as u32)
    }
}