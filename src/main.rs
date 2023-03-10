use std::vec;

use blockchainlib::*;

fn main(){
     let difficulty = 0x0fffffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(0, now(), String::from(""), vec![0;32], vec![0; 32],0, difficulty,
        vec![Transaction {
            tx_in: vec![],
            tx_out: vec![
                blockchainlib::TxOut {
                    address: "Paco".to_owned(),
                    amount: 10,
                },
            ],
        }], );

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new(difficulty);

    blockchain.update(genesis_block).expect("Failed to add genesis block");

}