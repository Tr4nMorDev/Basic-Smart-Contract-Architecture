use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug)]
pub struct Contract {
    pub id : u32,
    pub name : String ,
    pub sodu : u128,
    pub status : ContractStatus,
    pub created_at: u64,
}
#[derive(Debug,PartialEq)]
pub enum ContractStatus {
    Active,
    Frozen,
    Terminated
}

pub trait ContractAction {
    fn new (id : u32 , name : String , sodu : u128) -> Self ;
}
impl ContractAction for Contract {
    fn new (id : u32 , name : String , sodu : u128) -> Self {
        Self { 
            id ,
            name , 
            sodu,
            status : ContractStatus::Active,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}