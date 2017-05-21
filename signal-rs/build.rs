extern crate protobuf_build;

use std::env;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let source = root.join("protobuf");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut compiler = protobuf_build::Compiler::new(&source, &out);

    compiler.compile("SignalService.proto").unwrap();
    compiler.compile("WhisperTextProtocol.proto").unwrap();
    compiler.compile("LocalStorageProtocol.proto").unwrap();
}
