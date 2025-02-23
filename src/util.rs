use parity_scale_codec::{Decode, Encode, MaxEncodedLen};

use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, scale_info::TypeInfo)]
pub struct RuntimeBoundedVec {
    inner: Vec<u8>,
    max_len: u32, // Store the maximum length as a field
}

impl RuntimeBoundedVec {
    pub fn new(vec: Vec<u8>, max_len: u32) -> Result<Self, &'static str> {
        if vec.len() > max_len as usize {
            Err("Vector exceeds maximum length")
        } else {
            Ok(Self {
                inner: vec,
                max_len,
            })
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn max_len(&self) -> u32 {
        self.max_len
    }
}

impl MaxEncodedLen for RuntimeBoundedVec {
    fn max_encoded_len() -> usize {
        // Use a conservative upper bound for the encoded length
        1024 + 4 // 1024 bytes for the vector + 4 bytes for the length prefix
    }
}
