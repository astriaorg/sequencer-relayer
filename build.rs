fn main() {
    println!("cargo:rerun-if-changed=src/proto/msg.proto");
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/proto")
        .inputs(["src/proto/msg.proto"])
        .include("src/proto")
        .run()
        .expect("Codegen failed.");
}
