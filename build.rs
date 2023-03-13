fn main() {
    println!("cargo:rerun-if-changed=proto/");
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/proto")
        .inputs(["proto/msg.proto", "proto/tx.proto"])
        .include("proto")
        .run()
        .expect("Codegen failed.");
}
