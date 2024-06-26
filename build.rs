extern crate capnpc;

fn main() {
    capnpc::CompilerCommand::new()
        .output_path("src/")
        .src_prefix("src/schemas/")
        .file("src/schemas/test_schema.capnp")
        .run()
        .unwrap();
}
