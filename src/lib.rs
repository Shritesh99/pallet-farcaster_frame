#![cfg_attr(not(feature = "std"), no_std)]
// pub use pallet::*;

#[cfg(test)]
mod tests;

pub mod message;
use message::*;
use parity_scale_codec::{Decode, Encode};
use sp_std::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidProtobuf,
    InvalidMessage,
}

/// Decodes a SCALE-encoded `Message` from a raw byte vector.
///
/// # Arguments
///
/// * `raw` - A `Vec<u8>` containing the SCALE-encoded message.
///
/// # Returns
///
/// * `Ok(Message)` if decoding is successful.
/// * `Err(Error::InvalidProtobuf)` if decoding fails due to an invalid byte sequence.
///
/// # Example
///
/// ```
/// use pallet_farcaster_frame::{parse_message, Error};
/// use sp_std::vec;
///
/// let invalid_bytes = vec![];
/// let result = parse_message(invalid_bytes);
/// assert!(result.is_err());
/// ```
pub fn parse_message(raw: Vec<u8>) -> Result<Message, Error> {
    let msg = Message::decode(&mut &*raw).map_err(|_| Error::InvalidProtobuf)?;
    Ok(msg)
}

/// Encodes a `Message` into a SCALE-encoded byte vector.
///
/// # Arguments
///
/// * `msg` - A reference to a `Message` struct to be encoded.
///
/// # Returns
///
/// * `Ok(Vec<u8>)` containing the SCALE-encoded data if encoding is successful.
/// * `Err(Error::InvalidMessage)` if encoding results in an empty byte vector.
///
/// # Example
///
/// ```
/// use pallet_farcaster_frame::{encode_message, message::Message};
///
/// let msg = Message {
///     data: None,               // No valid data
///     hash: Vec::new(),         // Empty hash
///     hash_scheme: 0,           // Potentially invalid scheme
///     signature: Vec::new(),    // Empty signature
///     signature_scheme: 0,      // Invalid scheme
///     signer: Vec::new(),       // No signer info
///     data_bytes: None,         // No raw data
///  };
/// let encoded = encode_message(&msg);
/// assert!(encoded.is_ok());
/// ```
pub fn encode_message(msg: &Message) -> Result<Vec<u8>, Error> {
    let encoded = msg.encode();
    if encoded.is_empty() {
        Err(Error::InvalidMessage)
    } else {
        Ok(encoded)
    }
}
