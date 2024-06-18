use std::collections::HashSet;
use crate::*;

#[derive(Clone)]
pub struct Transfer {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashable for Transfer {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

pub struct Transaction {
    pub inputs: Vec<Transfer>,
    pub outputs: Vec<Transfer>
}

impl Transaction {

    pub fn input_value(&self) -> u64 {
        Transaction::sum_values(&self.inputs)
    }
    pub fn output_value(&self) -> u64 {
        Transaction::sum_values(&self.outputs)
    }
    pub fn input_hashes (&self) -> HashSet<BlockHash> {
        Transaction::get_hash_set(&self.inputs)
    }
    pub fn output_hashes (&self) -> HashSet<BlockHash> {
        Transaction::get_hash_set(&self.outputs)
    }

    // transaction is a coinbase if it doesnt have any input
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }



    // helper function to sum values in vector of outputs/inputs
    pub fn sum_values (v: &Vec<Transfer>) -> u64{
        v.iter().map(|element| element.value).sum()
    }

     // set of hashes
    pub fn get_hash_set(v: &Vec<Transfer>) -> HashSet<BlockHash> {
        v
        .iter()
        .map(|element| element.hash())
        .collect::<HashSet<BlockHash>>()
    }



}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(
            // flat map appends vectors of bytes together to one iterator
            // next transform this iterator to collection
            self.inputs.iter().
            flat_map(|input| input.bytes()).
            collect::<Vec<u8>>()
        );
        bytes.extend(
            self.outputs.iter().
            flat_map(|input| input.bytes()).
            collect::<Vec<u8>>()
        );
        bytes
    }
}