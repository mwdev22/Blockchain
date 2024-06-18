#![allow(unused)]
use std::collections::HashSet;

use blockchain_from_scratch::{ now, Block, BlockChain, BlockHash, Hashable, Transfer, Transaction};


fn main() {
    let difficulty = 0x000ffffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0; 32], 0,vec![Transaction {
            inputs: vec![ ],
            outputs: vec![
                Transfer {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                Transfer {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        },
    
    ], difficulty);
    // displaying the debug fmt implementation
    // block.mine() // bardzo ciężka operacja

    let mut last_hash = block.hash.clone();

    let mut blockchain = BlockChain {
        blocks: vec![block],
        unspent_outputs: HashSet::<BlockHash>::new()
    };

    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash , 0, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                Transfer {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                Transfer {
                    to_addr: "Alice".to_owned(),
                    value: 360,
                },
                Transfer {
                    to_addr: "Bob".to_owned(),
                    value: 12,
                },
            ],
        },
    ], difficulty);
        // displaying the debug fmt implementation
        println!("{:?}", &block);
        let h = block.hash();
        block.mine();
        println!("mined: {:?} hash", &block);
        last_hash = block.hash.clone();
        blockchain.update_with_block(block).expect("Failed to add block");
    }
    
    // TODO transactions logic

}
