use crate::*;

#[derive(Debug, Clone)]
pub struct Wallet {
    private_key: String,
    public_key: String,
    balance: f64
} 

impl Hashable for Wallet {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.private_key.as_bytes());
        bytes.extend(self.public_key.as_bytes());
        bytes.extend(self.balance.to_be_bytes());
        bytes
    }
}