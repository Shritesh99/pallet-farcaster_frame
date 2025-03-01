# Farcaster Frames Pallet for Polkadot and Substrate Ecosystem

## Overview

This repository contains a custom Substrate pallet designed for integrating Farcaster frames into Polkadot/Substrate-based blockchains. The pallet decodes Farcaster messages, allowing developers to incorporate social media interactions into blockchain applications. As a demonstration, the pallet is integrated with a simple NFT minting functionality, enabling on-chain asset creation driven by Farcaster data.

Example implmentation in pallet can be found here at [lib.rs](https://github.com/Shritesh99/polkadot-farcaster-demo-parachain/blob/master/pallets/template/src/lib.rs).

## Features

-    **Farcaster Message Decoding**: Decodes Farcaster frames submitted as raw messages.
-    **Event Emission**: Emits events such as `MessageValidated` upon successful processing, enabling real-time feedback and off-chain indexing.

## Builoding locally

### Prerequisites

-    Rust (stable)
-    Substrate (latest recommended version)
-    Cargo

### Installation in your pallet

1. Add to your pallet dependencies

```
cargo add pallet-farcaster_frame
```

2. Import the module

```
use pallet_farcaster_frame::{parse_message, Error};
```

### Usage

1. Parse the farcaster message

```rust
use pallet_farcaster_frame::{parse_message, Error};
use sp_std::vec;

let bytes = vec![];
let result: Result<Message, Error> = parse_message(bytes);
```

2. Encode the farcaster message

```rust
use pallet_farcaster_frame::{encode_message, message::Message};
let msg = Message {
...
}
let encoded = encode_message(&msg);
assert!(encoded.is_ok());
```

### Building locally

1. **Clone the repository:**

     ```bash
     git clone https://github.com/shritesh99/farcaster-frames-pallet.git
     cd farcaster-frames-pallet
     ```

2. **Build the project:**

     ```bash
     cargo build --release
     ```

3. **Run tests:**

     ```bash
     cargo test
     ```

## Usage

### Submitting a Farcaster Message

The pallet exposes the `submit_message` extrinsic, which accepts a raw message (a `Vec<u8>`) and processes it through the Farcaster message decoding logic. On successful decoding, it emits a `MessageValidated` event with the decoded message.

Example usage in a runtime or integration test:

```rust
// Assuming a valid encoded message exists:
let raw_message = /* your encoded message */;
assert_ok!(Pallet::<TestRuntime>::submit_message(Origin::signed(1), raw_message));
```

## PalletFarcasterInterface Trait

```rust
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
```

## Project Structure

-    **src/lib.rs**: Main pallet implementation including extrinsics, message processing, and integration logic.
-    **src/tests.rs**: Unit tests to verify the functionality of the pallet.
-    **Cargo.toml**: Configuration file for Rust package management.
-    Additional configuration and documentation files.

## Contributing

Contributions are welcome! Feel free to fork the repository, make enhancements, and submit pull requests. For major changes, please open an issue to discuss your ideas first.

## License

This project is licensed under the Apache 2.0 License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions, support, or collaboration inquiries, please contact Shritesh at [shritesh.sj@gmail.com].

---

This README provides a concise overview of the project, outlines its key features and setup instructions, and invites contributions from the community. Enjoy building with Farcaster Frames!
