use crate::{error::TicketMachineError::InvalidInstruction};
use solana_program::{msg, program_error::ProgramError};
use std::convert::TryInto;
use std::fmt::format;
pub enum TicketSellingPoolInstructions {
    ///Init a ticket pool
    /// 0.`[writable,signer]` Create a new pool with this account, this will be the pool id
    /// 1.`[writable,signer]` Sol account that is the manager to receive the payment
    /// 2.`[writable]` Fee receiver const "2wnEcArzCpX1QRdtpHRXxZ7k9b1UeK16mPt26LPWFZ6V"
    InitPool {
        price: u64,        //in Lamport
        fee: u8,           //in percentage must at least 5%
        total_amount: u64, //total amount you wanted to sell
    },
    /// 0.`[writable]` pool id
    /// 1.`[writable]` Sol account to receive the payment
    /// 2.`[writable]` Fee receiver const "2wnEcArzCpX1QRdtpHRXxZ7k9b1UeK16mPt26LPWFZ6V"
    /// 3.`[writable,signer]` Ticket account
    /// 4.`[writable,signer]` Wallet to Pay Sol and buy a Ticket
    /// 5.`[]`System Program
    Buy {},
}
impl TicketSellingPoolInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => {
                let (price, rest) = Self::unpack_u64(rest).unwrap();
                let (fee, rest) = Self::unpack_u8(rest).unwrap();
                let amount = Self::unpack_u64(rest).unwrap().0;
                let message = format(format_args!(
                    "price: {:?} fee: {:?} amount: {:?}",
                    price, fee, amount
                ));
                msg!(&message);
                Self::InitPool {
                    price: price,
                    fee: fee,
                    total_amount: amount,
                }
            }
            1 => Self::Buy {},
            _ => return Err(InvalidInstruction.into()),
        })
    }
    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        let (amount, rest) = input.split_at(8);
        let amount = amount
            .try_into()
            .ok()
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok((amount, rest))
    }
    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        let (amount, rest) = input.split_at(1);
        let amount = amount
            .try_into()
            .ok()
            .map(u8::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok((amount, rest))
    }
}
