#![cfg_attr(not(feature = "std"), no_std)]
// pub use pallet::*;

#[cfg(test)]
mod tests;

pub mod message;

pub mod pallet_farcaster {
    use crate::message::*;
    use sp_std::vec::Vec;
    #[derive(Debug)]
    pub enum Error {
        InvalidProtobuf,
    }
    pub fn parse_message(raw: Vec<u8>) -> Result<Message, Error> {
        let msg = Message::decode(&mut &*raw).map_err(|_| Error::InvalidProtobuf)?;
        Ok(msg)
    }
}
