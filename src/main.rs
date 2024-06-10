#![allow(unused)]
use blockchain_from_scratch::{blockchain::hashable::Hashable, now, Block};


fn main() {
    let mut block = Block::new(0, now(), vec![0; 32], 0, "first block".to_owned(), 0x00ffffffffffffff);
    // displaying the debug fmt implementation
    println!("{:?}", &block);
    let h = block.hash();
    println!("{:?}", &h);
    block.hash = h; // assign hash to the block
    block.mine()
}
