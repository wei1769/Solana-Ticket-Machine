use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    msg,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
pub struct Pool {
    pub account_type: u8,//1 is pool size:1
    pub manager: Pubkey,//size:32
    pub fee_reciever: Pubkey,//size:32
    pub total_amount: u64,//size:8
    pub price:  u64,//size:8
    pub fee: u8,//size:1
    pub current_number: u64//size:8

    //Pool account size should be 90 Bytes
}
pub struct Ticket{
    pub account_type: u8,//2 is Ticket size:1
    pub pool_id: Pubkey,//size:32
    pub ticketnumber:u64,//size:8
    pub ticketbuyer: Pubkey,//size:32

    //Ticket account size should be 73 Bytes
    
}



impl Sealed for Ticket {}
impl IsInitialized for Ticket{
    fn is_initialized(&self) -> bool {
        !self.account_type == 0 
    }
}

impl Pack for Ticket {
    const LEN: usize = 73;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        msg!("unpack ok");
        let src = array_ref![src, 0, Ticket::LEN];
        let(_account_type,
            _pool_id,
            _ticketnumber,
            _ticketbuyer,
            
        ) = array_refs![src, 1, 32, 8, 32];
        
        let account_type =u8::from_le_bytes(*_account_type);
        
        let pool_id = Pubkey::new(_pool_id);
        let ticketbuyer = Pubkey::new(_ticketbuyer);
        let ticketnumber = u64::from_le_bytes(*_ticketnumber);
        Ok(Ticket{
            account_type,
            pool_id,
            ticketnumber,
            ticketbuyer,
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Ticket::LEN];
        let(_account_type_dst,
            _pool_id_dst,
            _ticketnumber_dst,
            _ticketbuyer_dst,
            
        ) = mut_array_refs![dst, 1, 32, 8, 32];

        let Ticket{
            account_type,
            pool_id,
            ticketnumber,
            ticketbuyer,
            
        } = self;
        _account_type_dst[0] = *account_type as u8;
        _pool_id_dst.copy_from_slice(pool_id.as_ref());
        _ticketbuyer_dst.copy_from_slice(ticketbuyer.as_ref());
        *_ticketnumber_dst = ticketnumber.to_le_bytes();




    }
}
impl Sealed for Pool {}

impl IsInitialized for Pool{
    fn is_initialized(&self) -> bool {
        !self.account_type == 0 
    }
}

impl Pack for Pool {
    const LEN: usize = 90;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        msg!("unpack ok");
        let src = array_ref![src, 0, Pool::LEN];
        let(_account_type,
            _manager,
            _fee_reciever,
            _total_amount,
            _price,
            _fee,
            _current_number
        ) = array_refs![src, 1, 32, 32, 8, 8,1,8];
        
        let account_type =u8::from_le_bytes(*_account_type);
        
        let manager = Pubkey::new(_manager);
        let fee_reciever = Pubkey::new(_fee_reciever);
        let total_amount = u64::from_le_bytes(*_total_amount);
        let price = u64::from_le_bytes(*_price);
        let fee = u8::from_le_bytes(*_fee);
        let current_number = u64::from_le_bytes(*_current_number);
        Ok(Pool{
            account_type,
            manager,
            fee_reciever,
            total_amount,
            price,
            fee,
            current_number,
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Pool::LEN];
        let(_account_type_dst,
            _manager_dst,
            _fee_reciever_dst,
            _total_amount_dst,
            _price_dst,
            _fee_dst,
            _current_number_dst
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8,1,8];

        let Pool{
            account_type,
            manager,
            fee_reciever,
            total_amount,
            price,
            fee,
            current_number,
        } = self;
        _account_type_dst[0] = *account_type as u8;
        _manager_dst.copy_from_slice(manager.as_ref());
        _fee_reciever_dst.copy_from_slice(fee_reciever.as_ref());
        *_total_amount_dst = total_amount.to_le_bytes();
        *_price_dst = price.to_le_bytes();
        *_fee_dst = fee.to_le_bytes();
        *_current_number_dst = current_number.to_le_bytes();





    }
}