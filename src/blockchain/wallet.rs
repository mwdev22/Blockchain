use std::fmt::Debug;

use crate::*;
use rsa::{pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},  RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};


#[derive(Clone)]
pub struct Wallet {
    username: String,
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
    balance: f64
}

impl Wallet {
    pub fn new(username: String) -> Self {
        let (private_key, public_key) = Self::generate_keys();
        Wallet {
            username,
            private_key,
            public_key,
            balance: 0.0,
        }
    }
    fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        (private_key, public_key)
    }

    pub fn reset_keys(&mut self) {
        let (private_key, public_key) = Self::generate_keys();
        self.private_key = private_key;
        self.public_key = public_key;
    }

    pub fn set_balance(&mut self, balance: f64) {
        self.balance = balance;
    }

}

impl Hashable for Wallet {
    fn bytes(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.private_key.to_pkcs1_der().unwrap().as_bytes());
        hasher.update(self.public_key.to_pkcs1_der().unwrap().as_bytes());
        hasher.update(&self.balance.to_be_bytes());
        hasher.finalize().to_vec()
    }
}

impl Debug for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}'s wallet with balance {}", &self.username, &self.balance)
    }
}