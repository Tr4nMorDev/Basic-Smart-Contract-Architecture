pub struct Transaction {
    pub id : u32 ,
    pub from_id : u32 , 
    pub to_id : u32 ,
    pub amount : u128 ,
    pub transaction : TransactionStatus
}

pub enum TransactionStatus {
    Pending ,
    Completed ,
    Failed
}
