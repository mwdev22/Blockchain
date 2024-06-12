#![allow(unused)]
use blockchain_from_scratch::{ now, Block, BlockChain, Hashable};


fn main() {
    let difficulty = 0x000ffffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0; 32], 0, "first block".to_owned(), difficulty);
    // displaying the debug fmt implementation
    // block.mine() // bardzo ciężka operacja

    let mut last_hash = block.hash.clone();

    let mut blockchain = BlockChain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash , 0, format!("block {}", i).to_owned(), difficulty);
        // displaying the debug fmt implementation
        println!("{:?}", &block);
        let h = block.hash();
        block.mine();
        println!("mined: {:?} hash", &block);
        last_hash = block.hash.clone();
        blockchain.blocks.push(block);
    }
}
