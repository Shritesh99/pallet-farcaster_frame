#![cfg_attr(not(feature = "std"), no_std)]

mod message; // Generated protobuf code
mod username_proof; // Generated protobuf code

pub use ed25519_dalek::{SecretKey, Signer, SigningKey};
use hex::FromHex;
use reqwest::Client;

use message::{CastAddBody, FarcasterNetwork, MessageData};
pub use protobuf::Message;

pub use pallet::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		FarcasterVerification{ who: T::AccountId }
	}

	#[pallet::error]
	pub enum Error<T> {
		VerificationFailed
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(Weight::default())]
		#[pallet::call_index(1)]
		pub fn verify(origin: OriginFor<T>, _message: Message){
			
			Ok(())
		}
	}
}
