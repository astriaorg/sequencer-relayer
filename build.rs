fn main() {
    println!("cargo:rerun-if-changed=src/proto/*.proto");
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/proto")
        .inputs(["src/proto/msg.proto", "src/proto/tx.proto"])
        .include("src/proto")
        .run()
        .expect("Codegen failed.");
}
