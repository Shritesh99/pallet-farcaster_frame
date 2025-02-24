use crate::{message::*, mock::*, Event};
use base64::prelude::*;
use frame_support::assert_ok;
use hex_literal::hex;

// Convert an ISO8601 time to a u32 timestamp (in seconds).
// Here we hardcode the computed value for "1973-02-28T09:13:52Z".
const TIMESTAMP: u32 = 99_825_232;

fn generate_message() -> Message {
    // Build CastId for the FrameActionBody.
    let cast_id = CastId {
        fid: 289309,
        // Decode the hex string "0x0000000000000000000000000000000000000001"
        hash: hex!("0000000000000000000000000000000000000001").to_vec(),
    };

    // Build the FrameActionBody.
    let frame_action_body = FrameActionBody {
        // Decode the base64-encoded URL.
        url: BASE64_STANDARD
            .decode("aHR0cHM6Ly9wZWxpY2FuLWZvbmQtZGlzdGluY3RseS5uZ3Jvay1mcmVlLmFwcC9vZw==")
            .expect("Invalid base64"),
        button_index: 1,
        cast_id: Some(cast_id),
        // For simplicity, we convert empty strings to empty byte vectors.
        input_text: Vec::new(),
        state: Vec::new(),
        transaction_id: Vec::new(),
        address: Vec::new(),
    };

    // Build MessageData.
    let message_data = MessageData {
        // Use the value for MESSAGE_TYPE_FRAME_ACTION (which is defined as 13).
        r#type: MessageType::FrameAction as i32,
        fid: 289309,
        timestamp: TIMESTAMP,
        // FARCASTER_NETWORK_MAINNET is 1.
        network: FarcasterNetwork::Mainnet as i32,
        // Wrap our frameActionBody inside the oneof as the FrameActionBody variant.
        body: Some(message_data::Body::FrameActionBody(frame_action_body)),
    };

    // Build the top-level Message.
    Message {
        data: Some(message_data),
        // Decode hex "0x6357261fa893e4be85f78178babaca876f9a1fac"
        hash: hex!("6357261fa893e4be85f78178babaca876f9a1fac").to_vec(),
        // HASH_SCHEME_BLAKE3 is value 1.
        hash_scheme: HashScheme::Blake3 as i32,
        // Decode the base64 signature.
        signature: BASE64_STANDARD.decode("0e1kmWQBg3dkGnhjjwwZ08NGwesaR+hWwPzYfT/HL/mBcvk5/Bj/3RavdGFEJ55t67P0kT9JHGnSL2cD5VRRCg==")
            .expect("Invalid base64 signature"),
        // SIGNATURE_SCHEME_ED25519 is value 1.
        signature_scheme: SignatureScheme::Ed25519 as i32,
        // Decode hex for signer.
        signer: hex!("0295183aaa021cad737db7ddbc075964496ece1c0bcc1009bdae6d1799c83cd4").to_vec(),
        // In this example, no alternate data_bytes are provided.
        data_bytes: None,
    }
}

#[test]
fn submit_message_should_work() {
    new_test_ext().execute_with(|| {
        // Create a dummy message with content "Hello, Farcaster!"
        let message = generate_message();
        // // Encode the message to a raw vector.
        let raw = message.encode();

        System::set_block_number(1);

        // Call the submit_message extrinsic with a signed origin.
        assert_ok!(PalletFarcaster::submit_message(
            RuntimeOrigin::signed(1),
            raw.clone()
        ));

        // Verify that the MessageValidated event was emitted with the expected message.
        System::assert_last_event(
            Event::MessageValidated {
                message: message.clone(),
            }
            .into(),
        );
    });
}
// {
//   "data": {
//     "FarcasterValidateFrameMessage": {
//       "message": {
//         "data": {
//           "fid": 289309,
//           "frameActionBody": {
//             "buttonIndex": 1,
//             "castId": {
//               "fid": 289309,
//               "hash": "0x0000000000000000000000000000000000000001"
//             },
//             "inputText": "",
//             "state": "",
//             "address": "",
//             "inputTextDecoded": "",
//             "stateDecoded": null,
//             "transactionHash": "0x",
//             "transactionId": "",
//             "url": "aHR0cHM6Ly9wZWxpY2FuLWZvbmQtZGlzdGluY3RseS5uZ3Jvay1mcmVlLmFwcC9vZw==",
//             "urlDecoded": "https://pelican-fond-distinctly.ngrok-free.app/og"
//           },
//           "network": "FARCASTER_NETWORK_MAINNET",
//           "time": "1973-02-28T09:13:52Z",
//           "type": "MESSAGE_TYPE_FRAME_ACTION"
//         },
//         "hash": "0x6357261fa893e4be85f78178babaca876f9a1fac",
//         "hashScheme": "HASH_SCHEME_BLAKE3",
//         "signature": "0e1kmWQBg3dkGnhjjwwZ08NGwesaR+hWwPzYfT/HL/mBcvk5/Bj/3RavdGFEJ55t67P0kT9JHGnSL2cD5VRRCg==",
//         "signatureScheme": "SIGNATURE_SCHEME_ED25519",
//         "signer": "0x0295183aaa021cad737db7ddbc075964496ece1c0bcc1009bdae6d1799c83cd4"
//       }
//     }
//   }
// }
// https://app.airstack.xyz/query/A8jvLznU9P?_gl=1*nk2u9z*_ga*MTUyNzExOTYzNC4xNzQwNDE3NzYy*_ga_6PP294SC61*MTc0MDQxNzc2Mi4xLjEuMTc0MDQxNzg2MS4wLjAuMA..
