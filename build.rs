extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &[
            "protobufs/schemas/message.proto",
            "protobufs/schemas/username_proof.proto",
        ],
        &["protobufs/schemas/"],
    )
    .unwrap();
}
