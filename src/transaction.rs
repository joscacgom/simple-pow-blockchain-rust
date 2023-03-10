use super::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct TxIn {
   pub address: Address,
   pub amount: u64,
}
#[derive(Clone)]
pub struct TxOut {
    pub address: Address,
    pub amount: u64,
}

impl Hashable for TxOut {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.address.as_bytes());
        bytes.extend_from_slice(&self.amount.to_be_bytes());
        bytes
    }
}

impl Hashable for TxIn {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.address.as_bytes());
        bytes.extend_from_slice(&self.amount.to_be_bytes());
        bytes
    }
}

#[derive(Clone)]
pub struct Transaction {
    pub tx_in: Vec<TxIn>,
    pub tx_out: Vec<TxOut>,
}

impl Transaction {
    pub fn input_sum (&self) -> u64 {
        let mut sum = 0;
        for tx_in in &self.tx_in {
            sum += tx_in.amount;
        }
        sum
    }

    pub fn output_sum (&self) -> u64 {
        let mut sum = 0;
        for tx_out in &self.tx_out {
            sum += tx_out.amount;
        }
        sum
    }

    pub fn input_hash (&self) -> HashSet<Hash> {
        let set = HashSet::from_iter(self.tx_in.iter().
            map(|tx_in| tx_in.hash()).collect::
                <HashSet<Hash>>());
        set
       
    }

    pub fn output_hash (&self) -> HashSet<Hash> {
        let set = HashSet::from_iter(self.tx_out.iter().
            map(|tx_out| tx_out.hash()).collect::
                <HashSet<Hash>>());
        set
       
    }

    pub fn is_coinbase (&self) -> bool {
        self.tx_in.len() == 0
    }
}

impl Hashable for Transaction {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];
        for tx_in in &self.tx_in {
            bytes.extend_from_slice(&tx_in.bytes());

        }
        for tx_out in &self.tx_out {
            bytes.extend_from_slice(&tx_out.bytes());
        }
        bytes
    }
}