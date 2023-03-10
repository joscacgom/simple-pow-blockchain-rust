use crate::block::check_difficulty;

use super::*;
use std::collections::HashSet;


#[derive(Debug)]
pub enum ValidationError {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}
#[derive(Debug)]
pub struct Blockchain{
    pub blocks:Vec<Block>,
    pub difficulty:u128,
    unspent_outputs: HashSet<Hash>,

}

impl Blockchain{
    pub fn new(difficulty:u128)->Self{
        let mut blocks = Vec::new();
        blocks.push(Block::new(0,0,"Genesis Block".to_string(),vec![0;32],vec![0;32],0, difficulty,vec![]));
        Blockchain{
            blocks,
            difficulty,
            unspent_outputs: HashSet::new(),
        }
    }

    pub fn update(&mut self, block: Block) -> Result<(), ValidationError> {
       let i = self.blocks.len() - 1;
        if block.id != i as u32 {
            return Err(ValidationError::MismatchedIndex);
        } else if block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(ValidationError::InvalidHash);
        } else if i != 0 {
            // Not genesis block
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(ValidationError::AchronologicalTimestamp);
            } else if block.previous_hash != prev_block.hash {
                return Err(ValidationError::MismatchedPreviousHash);
            }
        } else {
            // Genesis block
            if block.previous_hash != vec![0; 32] {
                return Err(ValidationError::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(ValidationError::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hash();

                if
                    !(&input_hashes - &self.unspent_outputs).is_empty() ||
                    !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(ValidationError::InvalidInput);
                }

                let input_value = transaction.input_sum();
                let output_value = transaction.output_sum();

                if output_value > input_value {
                    return Err(ValidationError::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hash());
            }

            if coinbase.output_sum() < total_fee {
                return Err(ValidationError::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hash());
            }

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);

        Ok(())
    }
}