use super::*;
use std::{fmt::{self,Debug,Formatter}, error::Error};

//clone
#[derive(Clone)]
pub struct Block{
    pub id:u32,
    pub timestamp:u128,
    pub data:String,
    pub previous_hash:BlockHash,
    pub hash:BlockHash,
    pub nonce:u64,
    pub difficulty:u128,
    pub transactions: Vec<Transaction>,

}

impl Block{
    pub fn new(id:u32,timestamp:u128,data:String,previous_hash:BlockHash,hash:BlockHash,nonce:u64, difficulty:u128,transactions:Vec<Transaction>)->Self{
        Block{
            id,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
            difficulty,
            transactions
            
        }
    }

    pub fn mine(&mut self){
        for nonce_attempt in 0..(u64::MAX){
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty){
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id.to_be_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes.extend_from_slice(&self.data.as_bytes());
        bytes.extend_from_slice(&self.previous_hash);
        bytes.extend_from_slice(&self.hash);
        bytes.extend_from_slice(&self.nonce.to_be_bytes());
        bytes.extend_from_slice(&self.difficulty.to_be_bytes());
        bytes.extend(self.transactions.iter().flat_map(|tx| tx.bytes()).collect::<Vec<u8>>());
        bytes
    }
}

pub fn check_difficulty (hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Block id: {} timestamp: {} data: {} previous_hash: {} hash: {} nonce: {} difficulty: {} transactions: {}]",
            &self.id,
            &self.timestamp,
            &self.data,
            &hex::encode(&self.previous_hash),
            &hex::encode(&self.hash),
            &self.nonce,
            &self.difficulty,
            &self.transactions.len()
        )
        .unwrap(); // or `.expect("Failed to format Block")`
        
        Ok(())
    }
}