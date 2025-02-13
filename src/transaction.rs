use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug)]
pub struct Transaction {
    pub id : u32 ,
    pub from_id : u32 , 
    pub to_id : u32 ,
    pub amount : u128 ,
    pub status : TransactionStatus,
    pub time_excute : u64
}
#[derive(Debug)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed
}

pub trait TransactionAction {
    fn new (id : u32 , from_id :u32 ,to_id : u32  , amount : u128 , status : TransactionStatus) -> Self ;
}
impl TransactionAction for Transaction {
    fn new(id: u32, from_id: u32, to_id: u32, amount: u128, status: TransactionStatus) -> Self {
        Self {
            id,
            from_id,
            to_id,
            amount,
            status, // Sử dụng giá trị truyền vào thay vì luôn là `Pending`
            time_excute: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}