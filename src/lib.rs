#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod message;
mod util;

#[frame_support::pallet]
pub mod pallet {
    use crate::message::*;
    use frame_support::{pallet_prelude::*, Blake2_128Concat};
    use frame_system::pallet_prelude::*;
    use sp_core::{ed25519, keccak_256, ByteArray, H160};
    use sp_std::vec::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // Registries (simplified)
    #[pallet::storage]
    pub type IdRegistry<T> = StorageMap<_, Blake2_128Concat, u64, H160>;

    // #[pallet::storage]
    // pub type KeyRegistry<T> = StorageMap<_, Blake2_128Concat, (u64, RuntimeBoundedVec), bool>;

    // #[pallet::storage]
    // pub type StorageRegistry<T> = StorageMap<_, Blake2_128Concat, u64, u32>;

    // CRDT Storages
    // #[pallet::storage]
    // pub type Casts<T> =
    //     StorageMap<_, Blake2_128Concat, (u64, RuntimeBoundedVec), RuntimeBoundedVec>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MessageValidated { fid: u64, message_type: MessageType },
        MessageRejected { fid: u64, reason: Error<T> },
    }

    #[pallet::error]
    #[derive(Clone, PartialEq)]
    pub enum Error<T> {
        InvalidProtobuf,
        InvalidHash,
        InvalidSignature,
        SignerNotAuthorized,
        FidNotRegistered,
        InsufficientStorage,
        InvalidTimestamp,
        InvalidMessageType,
        InvalidMessageData,
        InvalidHashScheme,
        InvalidSignatureScheme,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub fn submit_message(origin: OriginFor<T>, raw: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let msg = Message::decode(&mut &*raw).map_err(|_| Error::<T>::InvalidProtobuf)?;
            let data = msg.data.as_ref().ok_or(Error::<T>::InvalidMessageData)?;

            // Validate message
            Self::validate_message(&msg, data, msg.clone().data_bytes.unwrap())?;

            // Process message type
            // match data {
            //     MessageType::MESSAGE_TYPE_CAST_ADD => Self::process_cast_add(data.fid, msg),
            //     // Handle other types...
            //     _ => Err(Error::<T>::InvalidMessageType)?,
            // };

            Self::deposit_event(Event::MessageValidated {
                fid: data.fid,
                message_type: MessageType::try_from(data.r#type).unwrap_or(MessageType::None),
            });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn validate_message(
            msg: &Message,
            data: &MessageData,
            data_bytes: Vec<u8>,
        ) -> Result<(), Error<T>> {
            // Check FID registration
            let custody = IdRegistry::<T>::try_get(data.fid)
                .ok()
                .ok_or(Error::<T>::FidNotRegistered)?;

            // // Validate signer key
            // let valid_signer = KeyRegistry::<T>::try_get((data.fid, msg.signer.clone()))
            //     .ok()
            //     .ok_or(Error::<T>::SignerNotAuthorized)?;
            // ensure!(valid_signer, Error::<T>::SignerNotAuthorized);

            // // Check storage
            // let units = StorageRegistry::<T>::try_get(data.fid).unwrap_or(0);
            // ensure!(
            //     units >= Self::required_units(data),
            //     Error::<T>::InsufficientStorage
            // );

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

            // // Check timestamp within 10 minutes
            // let now = 10; // Replace with actual current time
            // ensure!(
            //     data.timestamp <= now + 600 && data.timestamp >= now - 600,
            //     Error::<T>::InvalidTimestamp
            // );

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

        fn process_cast_add(fid: u64, msg: Message) -> DispatchResult {
            // let data = msg.data.as_ref().ok_or(Error::<T>::InvalidMessageData)?;
            // let body = if let Some(MessageData_oneof_body::cast_add_body(b)) = &data.body {
            //     b
            // } else {
            //     return Err(Error::<T>::InvalidMessageType.into());
            // };

            // // CRDT resolution: Check existing casts and apply last-write-wins
            // let key = (fid, msg.hash.clone());
            // let existing = Casts::<T>::get(&key);
            // if existing.map_or(true, |e| data.timestamp > e.timestamp) {
            //     Casts::<T>::insert(&key, body.clone());
            // }

            Ok(())
        }

        fn required_units(data: &MessageData) -> u32 {
            // Determine storage units required based on message type
            1 // Simplified for example
        }
    }
}
