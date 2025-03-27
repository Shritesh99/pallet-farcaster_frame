use ppsc_build::Config;

fn main() {
    Config::new()
        .out_dir("src")
        .compile_protos(
            &["protobufs/schemas/message.proto"],
            &["protobufs/schemas/"],
        )
        .unwrap();
}
