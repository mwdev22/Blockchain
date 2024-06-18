use std::fmt::{ self,  Debug, Formatter };
use crate::*;
use hex::encode;


pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128
}


impl Block {
    pub fn new(
        index: u32,timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,transactions: Vec<Transaction>, difficulty: u128) -> Self{
            Block { index: index, timestamp: timestamp, hash: vec![0; 32], prev_block_hash: prev_block_hash, nonce: nonce, transactions: transactions, difficulty: difficulty }
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
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
        )
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
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>()
        );
        bytes.extend(&u128_bytes(&self.difficulty));        
        bytes
    }
}

