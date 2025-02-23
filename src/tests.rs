use super::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_core::{ed25519, Pair, H160};

// Helper to generate Ed25519 key pair
// fn ed25519_pair() -> ed25519::Pair {
//     ed25519::Pair::from_string("//Alice", None).unwrap()
// }

// // Helper to generate Ethereum address
// fn eth_address() -> H160 {
//     H160::from_slice(&hex!("d43593c715fdd31c61141abd04a99fd6822c8558"))
// }

// #[test]
// fn verify_ed25519_valid_signature() {
//     new_test_ext().execute_with(|| {
//         let pair = ed25519_pair();
//         let message = b"test_message";
//         let signature = pair.sign(message);

//         let result = Template::verify_signature(
//             SignatureScheme::Ed25519,
//             &pair.public().to_vec(),
//             message,
//             &signature.to_vec(),
//             &eth_address(), // Custody addr not used here
//         );

//         assert_ok!(result, true);
//     });
// }

// #[test]
// fn verify_ed25519_invalid_signature() {
//     let pair = ed25519_pair();
//     let mut signature = pair.sign(b"test").to_vec();
//     signature[0] = signature[0].wrapping_add(1); // Tamper with signature

//     let result = Template::verify_signature(
//         SignatureScheme::Ed25519,
//         &pair.public().to_vec(),
//         b"test",
//         &signature,
//         &eth_address(),
//     );

//     assert_ok!(result, false);
// }

// #[test]
// fn verify_eip712_valid_signature() {
//     let message_hash = hex!("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
//     let signature = hex!("1cdd551e3dc9934caedc85c6d50fde1b899babaae9a7e2e5b0db4a7d1343d7c6355d5a2a86ad8b5a0e34021c9a0374f6983b4780b94943d3c1f3e6f26d21a551c");
//     let custody_addr = H160::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac"));

//     let result = Pallet::<Test>::verify_signature(
//         SignatureScheme::Eip712,
//         &vec![], // Signer field unused
//         &message_hash,
//         &signature,
//         &custody_addr,
//     );

//     assert_ok!(result, true);
// }

// #[test]
// fn verify_eip712_wrong_address() {
//     let message_hash = hex!("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
//     let signature = hex!("1cdd551e3dc9934caedc85c6d50fde1b899babaae9a7e2e5b0db4a7d1343d7c6355d5a2a86ad8b5a0e34021c9a0374f6983b4780b94943d3c1f3e6f26d21a551c");
//     let wrong_addr = eth_address();

//     let result = Pallet::<Test>::verify_signature(
//         SignatureScheme::Eip712,
//         &vec![],
//         &message_hash,
//         &signature,
//         &wrong_addr,
//     );

//     assert_ok!(result, false);
// }

// #[test]
// fn verify_invalid_scheme() {
//     let result = Pallet::<Test>::verify_signature(
//         SignatureScheme::None,
//         &vec![],
//         &[],
//         &vec![],
//         &eth_address(),
//     );

//     assert_err!(result, Error::<Test>::InvalidSignatureScheme);
// }

// #[test]
// fn verify_eip712_malformed_signature() {
//     let invalid_sig = vec![0u8; 63]; // Too short

//     let result = Pallet::<Test>::verify_signature(
//         SignatureScheme::Eip712,
//         &vec![],
//         &[],
//         &invalid_sig,
//         &eth_address(),
//     );

//     assert_err!(result, Error::<Test>::InvalidSignature);
// }

// #[test]
// fn verify_ed25519_malformed_public_key() {
//     let invalid_pubkey = vec![0u8; 31]; // Incorrect length

//     let result = Pallet::<Test>::verify_signature(
//         SignatureScheme::Ed25519,
//         &invalid_pubkey,
//         &[],
//         &vec![0u8; 64],
//         &eth_address(),
//     );

//     assert_err!(result, Error::<Test>::InvalidSignature);
// }

// #[test]
// fn verify_eip712_invalid_recovery() {
//     let bad_sig = hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");

//     let result = Pallet::<Test>::verify_signature(
//         SignatureScheme::Eip712,
//         &vec![],
//         &[],
//         &bad_sig,
//         &eth_address(),
//     );

//     assert_err!(result, Error::<Test>::InvalidSignature);
// }
