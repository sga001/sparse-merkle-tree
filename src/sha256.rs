use crate::{traits::Hasher, H256};
use sha2::{Sha256, Digest};

pub struct Sha256Hasher(Sha256);

impl Default for Sha256Hasher {
    fn default() -> Self {
        Sha256Hasher(Sha256::new())
    }
}

impl Hasher for Sha256Hasher {
    fn write_h256(&mut self, h: &H256) {
        self.0.update(h.as_slice());
    }
    fn write_byte(&mut self, b: u8) {
        self.0.update(&[b][..]);
    }
    fn finish(self) -> H256 {
        let hash: [u8; 32] = self.0.finalize().into();
        hash.into()
    }
}
