#![allow(unused)]
use std::collections::HashSet;

use blockchain_from_scratch::{ now, Block, BlockChain, BlockHash, Hashable, Transfer, Transaction};


fn main() {
    let difficulty = 0x000ffffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0; 32], 0,vec![Transaction {
            inputs: vec![],
            outputs: vec![
                Transfer {
                    to_addr: "Alice".to_owned(),
                    value: 505,
                },
                Transfer {
                    to_addr: "Bob".to_owned(),
                    value: 724,
                },
            ],
        },
    
    ], difficulty);
    // displaying the debug fmt implementation
    // block.mine() // bardzo ciężka operacja

    
    block.mine();
    let mut last_hash = block.hash.clone();

    let mut blockchain = BlockChain::new();
    blockchain.update_with_block(block);

    for i in 1..=10 {
        let prev_block = &blockchain.blocks[i - 1];
        let mut block = Block::new(i as u32, now(), last_hash.clone(), 0, vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    Transfer {
                        to_addr: "Chris".to_owned(),
                        value: 536 + i as u64,
                    },
                ],
            },
            Transaction {
                inputs: vec![
                    prev_block.transactions[0].outputs[0].clone(),
                ],
                outputs: vec![
                    Transfer {
                        to_addr: "Alice".to_owned(),
                        value: 360 + i as u64,
                    },
                    Transfer {
                        to_addr: "Bob".to_owned(),
                        value: 12 + i as u64,
                    },
                ],
            },
        ], difficulty);
        
        // displaying the debug fmt implementation
        println!("{:?}", &block);
        block.mine();
        println!("mined: {:?} hash", &block.hash());
        last_hash = block.hash.clone();
        blockchain.update_with_block(block).expect("Failed to add block");
    }
    

}
