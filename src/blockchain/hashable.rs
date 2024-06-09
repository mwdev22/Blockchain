use crypto_hash::{digest, Algorithm::SHA256, };
pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> Vec<u8> {
        digest(SHA256, &self.bytes())
    }
}