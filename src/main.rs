#![allow(unused)]
use std::collections::HashSet;
use rand::Rng;
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

    let mut rng = rand::thread_rng();


    for i in 1..=10 {
        let prev_block = &blockchain.blocks[i - 1];
        let mut transactions = vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    Transfer {
                        to_addr: "Chris".to_owned(),
                        value: 500 + rng.gen_range(0..100), // random value
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
                        value: 300 + rng.gen_range(0..100), // random value
                    },
                    Transfer {
                        to_addr: "Bob".to_owned(),
                        value: 10 + rng.gen_range(0..10), // random value
                    },
                ],
            },
        ];

        let mut block = Block::new(i as u32, now(), last_hash.clone(), 0, transactions, difficulty);
        // displaying the debug fmt implementation
        println!("{:?}", &block);
        block.mine();
        println!("mined: {:?} hash", &block.hash());
        last_hash = block.hash.clone();
        blockchain.update_with_block(block).expect("Failed to add block");
    }
    println!("Total unspent amount: {}", blockchain.total_unspent());
    

}
