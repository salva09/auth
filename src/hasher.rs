use sha2::{Digest, Sha256};

pub fn hash(input: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(input);
    format!("{:X}", sha256.finalize())
}