use crate::message::*;

/// A trait defining the interface for Farcaster message handling within the pallet.
pub trait PalletFarcasterInterface<AccountId> {
    /// The error type returned by the trait functions.
    type Error;
    /// The type representing a Farcaster message.
    type Message;

    /// Processes a raw Farcaster message from the runtime and validates it.
    ///
    /// # Arguments
    /// * `raw` - The raw byte vector containing the message.
    /// * `sender` - The account ID of the sender submitting the message.
    ///
    /// # Returns
    /// A result containing the parsed and validated `Message` or an error of type `Self::Error`.
    fn submit_message_from_runtime(
        raw: Vec<u8>,
        sender: &AccountId,
    ) -> Result<Message, Self::Error>;
}
