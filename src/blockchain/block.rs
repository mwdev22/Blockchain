use std::fmt::{ self,  Debug, Formatter };
use crate::BlockHash;


pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    
}



impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "block")
    }
}

