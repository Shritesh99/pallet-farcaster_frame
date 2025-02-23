use crate::message::*;
pub trait PalletFarcasterInterface<AccountId> {
    type Error;
    type Message;
    fn submit_message_from_runtime(
        raw: Vec<u8>,
        sender: &AccountId,
    ) -> Result<Message, Self::Error>;
}
