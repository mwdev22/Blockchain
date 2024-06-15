use std::fmt::{ self,  Debug, Formatter };
use crate::{check_difficulty, u128_bytes, u32_bytes, u64_bytes, BlockHash, Hashable};
use hex::encode;


pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u128
}


impl Block {
    pub fn new(
        index: u32,timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,payload: String, difficulty: u128) -> Self{
            Block { index: index, timestamp: timestamp, hash: vec![0; 32], prev_block_hash: prev_block_hash, nonce: nonce, payload:payload, difficulty: difficulty }
        }
    pub fn mine (&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            // println!("attempt: {nonce_attempt}, hash: {:?}", hash);
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                println!("nonce: {nonce_attempt}, hash: {:?}", self.hash);
                
                return;
            }
        }
    }
}


impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Block[{}]: {}. at: {}, with: {}", self.index,encode(&self.hash), self.timestamp, self.payload)
    }
}


// generating bytes based on block data for hashing
impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&u128_bytes(&self.difficulty));        
        bytes
    }
}

