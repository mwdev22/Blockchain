
use crate::*;

pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>
}

impl Transaction {
    
}