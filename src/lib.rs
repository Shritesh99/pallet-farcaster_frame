#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod message;
mod pallet_farcaster;

#[frame_support::pallet]
pub mod pallet {
    use crate::message::*;
    use crate::pallet_farcaster::PalletFarcasterInterface;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_core::{ed25519, keccak_256, ByteArray, H160};
    use sp_std::vec::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MessageValidated { message: Message },
        MessageRejected { fid: u64, reason: Error<T> },
    }

    #[pallet::error]
    #[derive(Clone, PartialEq)]
    pub enum Error<T> {
        InvalidProtobuf,
        InvalidHash,
        InvalidSignature,
        InvalidHashScheme,
        InvalidSignatureScheme,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub fn submit_message(origin: OriginFor<T>, raw: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let msg =
                <Self as PalletFarcasterInterface<T::AccountId>>::submit_message_from_runtime(
                    raw, &sender,
                )?;

            Self::deposit_event(Event::MessageValidated { message: msg });
            Ok(())
        }
    }

    impl<T: Config> PalletFarcasterInterface<T::AccountId> for Pallet<T> {
        type Error = Error<T>;
        type Message = Message;

        fn submit_message_from_runtime(
            raw: Vec<u8>,
            sender: &T::AccountId,
        ) -> Result<Message, Self::Error> {
            Self::process_message(raw, sender)
        }
    }

    impl<T: Config> Pallet<T> {
        fn process_message(raw: Vec<u8>, sender: &T::AccountId) -> Result<Message, Error<T>> {
            // Decode the message.
            let msg = Message::decode(&mut &*raw).map_err(|_| Error::<T>::InvalidProtobuf)?;

            // Convert sender's AccountId into an Ethereum-style address.
            // let custody = account_to_eth_address::<T>(sender);

            // Validate the message using your existing logic.
            // Self::validate_message(&msg, raw, custody)?;

            Ok(msg)
        }

        fn validate_message(
            msg: &Message,
            data_bytes: Vec<u8>,
            custody: H160,
        ) -> Result<(), Error<T>> {
            // Verify hash
            let computed = Self::compute_hash(
                &data_bytes,
                HashScheme::from_i32(msg.hash_scheme).unwrap_or(HashScheme::None),
            )?;
            ensure!(computed == msg.hash, Error::<T>::InvalidHash);

            // Verify signature
            let valid = Self::verify_signature(
                SignatureScheme::from_i32(msg.signature_scheme).unwrap_or(SignatureScheme::None),
                &msg.signer,
                &msg.hash,
                &msg.signature,
                &custody,
            )?;
            ensure!(valid, Error::<T>::InvalidSignature);

            Ok(())
        }

        fn compute_hash(data: &[u8], scheme: HashScheme) -> Result<Vec<u8>, Error<T>> {
            match scheme {
                HashScheme::Blake3 => Ok(blake3::hash(data).as_bytes()[..20].to_vec()),
                _ => Err(Error::<T>::InvalidHashScheme),
            }
        }

        fn verify_signature(
            scheme: SignatureScheme,
            signer: &[u8],
            hash: &[u8],
            sig: &[u8],
            custody: &H160,
        ) -> Result<bool, Error<T>> {
            match scheme {
                SignatureScheme::Ed25519 => {
                    let pub_key = ed25519::Public::from_slice(signer)
                        .map_err(|_| Error::<T>::InvalidSignature)?;
                    let signature = ed25519::Signature::from_slice(sig)
                        .map_err(|_| Error::<T>::InvalidSignature)?;
                    Ok(sp_io::crypto::ed25519_verify(&signature, hash, &pub_key))
                }
                SignatureScheme::Eip712 => {
                    let recovered = sp_io::crypto::secp256k1_ecdsa_recover(
                        sig.try_into().unwrap(),
                        hash.try_into().unwrap(),
                    )
                    .map_err(|_| Error::<T>::InvalidSignature)?;
                    let addr = H160::from_slice(&keccak_256(&recovered)[12..32]);
                    Ok(&addr == custody)
                }
                _ => Err(Error::<T>::InvalidSignatureScheme),
            }
        }
    }

    fn account_to_eth_address<T: frame_system::Config>(account: &T::AccountId) -> H160 {
        // Encode the account (usually AccountId32) into a Vec<u8>
        let encoded = account.encode();
        // Compute the Keccak256 hash (32 bytes)
        let hash = keccak_256(&encoded);
        // Take the last 20 bytes (index 12..32) to form the Ethereum address
        H160::from_slice(&hash[12..])
    }
}
