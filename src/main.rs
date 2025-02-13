mod contract;
mod transaction;

use contract::{Contract, ContractStatus };
use contract::ContractAction;
use transaction::{Transaction, TransactionStatus};

use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::result::Result;
struct Blockchain {
    contracts: HashMap<u32, Contract>,
    transactions: Vec<Transaction>,
    next_contract_id: u32,
    next_transaction_id: u32,

}
impl Blockchain {
    fn new()->Self{
        Self {
            contracts : HashMap::new(),
            transactions : Vec::new(),
            next_contract_id : 1 ,
            next_transaction_id : 1 
        }
    }
    fn add_contract(&mut self , name : &str , balance : u128 ) -> Result<(),String> {
        if self.contracts.contains_key(&self.next_contract_id){
            return Err("Contract ID already exists".to_string());
        };
        let contract = Contract::new(self.next_contract_id , name.to_string() , balance );
        self.contracts.insert(self.next_contract_id, contract); // giá trị key của haskmap là 1 
        self.next_contract_id += 1 ;
        Ok(())
    }
    fn frozen_Contract(&mut self , id : &u32 ) -> Result<() , String>{
        if let Some(c) = self.contracts.get_mut(id){
            c.status == ContractStatus::Frozen;
            Ok(())
        }else {
            Err("Contract not found".to_string())   
        }
    }
    fn check_contract_status_Active(&mut self , id : &u32 ) -> bool {
        self.contracts.get(id).map_or(false, |contract| contract.status == ContractStatus::Active)
    }
    fn check_contract_status_Frozen(&mut self , id : &u32) -> bool {
        self.contracts.get(id).map_or(false, |contract| contract.status == ContractStatus::Frozen)
    }
    
    fn transfer(&mut self , from_id : u32 , to_id : u32 , balance : u128) -> Result<(),String>{
        
        if !self.check_contract_status_Active(&from_id) {
            return Err("The contracts is not active".to_string());
        }
        if !self.check_contract_status_Active(&to_id){
            return Err("Contracts is not active".to_string());
        }
        let from = self.contracts.get_mut(&from_id).ok_or("Sender contract not found")?;
        if from.sodu < balance {
            return Err("Not enough money in account".to_string());
        }
        from.sodu -= balance;

        let to = self.contracts.get_mut(&to_id).ok_or("Receiver contract not found")?;
        to.sodu += balance;
        let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
        self.transactions.push(Transaction { id: self.next_transaction_id, from_id: from_id, to_id: to_id, amount: balance, status:  TransactionStatus::Completed, time_excute: now });
        self.next_transaction_id += 1 ;
        Ok(())
    }
    fn freeze_contract(&mut self , pre : u32) -> Result<(),String>{
        let contract = self.contracts.get_mut(&pre).ok_or("Contract not found")?;
        if contract.status == ContractStatus::Frozen {
            return Err("Contract is already frozen".to_string());
        }
        let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
        let days_elapsed = (now - contract.created_at) / (60 * 60 * 24); // Chuyển giây sang ngày

        if days_elapsed == 0 {
            contract.status = ContractStatus::Frozen;
            return Ok(())
        } else {
            return Err(format!("Contract can only be frozen after 30 days ({} days passed)", days_elapsed))
        }
    }
    fn display_contracts(&self) {
        for (i,a) in self.contracts.iter(){
            println!("i : {} , contract : {:?}",i,a);
        }
    }
    fn display_transactions(&self){
        for (i,a) in self.transactions.iter().enumerate(){
            println!("{:?}" , a)
        }
    }
    // fn display_transactions();
}



fn main() -> Result<() , Box<dyn Error>>{
    let mut blockchain = Blockchain::new();
    blockchain.add_contract("Alice", 1000)?;
    blockchain.add_contract("Bob", 500)?;
    blockchain.display_contracts();
    blockchain.transfer(1, 2, 200)?; // Alice gửi 200 cho Bob
    blockchain.freeze_contract(1)?;

    blockchain.display_contracts();
    blockchain.display_transactions();
    // blockchain.display_transactions();
    Ok(())

}
