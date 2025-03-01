extern crate prost_build;

fn main() {
    let mut config = prost_build::Config::new();
    config.type_attribute(
        ".",
        "#[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo)]",
    );
    config.out_dir("src");
    config
        .compile_protos(
            &["protobufs/schemas/message.proto"],
            &["protobufs/schemas/"],
        )
        .unwrap();
}
