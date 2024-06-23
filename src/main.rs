
#![allow(unused)]
use rand::Rng;
use blockchain_from_scratch::{now, Block, BlockChain, BlockHash, Hashable, Transfer, Transaction, Wallet};


fn main() {
    let difficulty = 0x000ffffffffffffffffffffffffffffff;

    // create wallets
    let mut maciek_wallet = Wallet::new("Alice".to_string());
    let mut john_wallet = Wallet::new("Bob".to_string());
    let mut chris_wallet = Wallet::new("Chris".to_string());


    maciek_wallet.set_balance(505.0);
    john_wallet.set_balance(724.0);

    let mut block = Block::new(0, now(), vec![0; 32], 0, vec![Transaction {
        inputs: vec![],
        outputs: vec![
            Transfer {
                receiver: maciek_wallet.clone(),
                value: 505,
            },
            Transfer {
                receiver: john_wallet.clone(),
                value: 724,
            },
        ],
    }], difficulty);

    block.mine();
    let mut last_hash = block.hash.clone();

    let mut blockchain = BlockChain::new();
    blockchain.update_with_block(block).expect("error adding first block");

    let mut rng = rand::thread_rng();

    for i in 1..=10 {
        let prev_block = &blockchain.blocks[i - 1];
        let transactions = vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    Transfer {
                        receiver: chris_wallet.clone(),
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
                        receiver: maciek_wallet.clone(),
                        value: 300 + rng.gen_range(0..100), // random value
                    },
                    Transfer {
                        receiver: john_wallet.clone(),
                        value: 10 + rng.gen_range(0..10), // random value
                    },
                ],
            },
        ];

        let mut block = Block::new(i as u32, now(), last_hash.clone(), 0, transactions, difficulty);
        println!("{:?}", &block);
        block.mine();
        println!("mined: {:?} hash", &block.hash());
        last_hash = block.hash.clone();
        blockchain.update_with_block(block).expect("Failed to add block");
    }
    println!("Total unspent amount: {}", blockchain.total_unspent());
}
