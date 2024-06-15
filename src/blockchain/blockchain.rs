use crate::*;


pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            // verify indexes
            if block.index != i as u32 {
                println!("Index missmatch {} != {}", &block.index, &i);
                false;
            // verify if block was mined properly by checking its difficulty
            } else if !check_difficulty(&block.hash(), block.difficulty){
                println!("difficulty fail");
                false;
            // validating hashes and timestamps for blocks
            } else if i==0 {
                if block.prev_block_hash != vec![0;32] {
                    println!("first block start prev hash invalid");
                }
            } else {
                let prev_block = &self.blocks[i-1];
                if block.timestamp <= prev_block.timestamp {
                    println!("time didnt increase between blocks {} and {}", i-1, i);
                }else if block.prev_block_hash != prev_block.hash {
                    println!("Prev block hash missmatch");
                }
            }
        }
        true
    }
}

