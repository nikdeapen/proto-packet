use proto_packet_compile::Compiler;
use proto_packet_gen::rust::RustGenerator;

#[test]
pub fn compile_rust() {
    let schema_dir: &'static str =
        "/Users/nicholasdeapen/src/proto-packet/proto-packet-compile/tests/compile_rust/schema/";
    let target_dir: &'static str =
        "/Users/nicholasdeapen/src/proto-packet/proto-packet-compile/tests/compile_rust/target/";

    let compiler: Compiler = Compiler::from(RustGenerator::default());
    compiler.compile(schema_dir, target_dir).unwrap();
}
