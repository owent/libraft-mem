use std::path::Path;
use std::fs;

// extern crate protoc_rust;
extern crate protobuf_codegen_pure;

fn main() {
    let output_dir = Path::new("src/protos");
    if !output_dir.exists() {
        let _ = fs::create_dir_all(output_dir).unwrap();
    }

    // protoc_rust::Args::new()
    //     .out_dir("src/protos")
    //     .include("protos")
    //     .input("protos/libraft-mem.proto")
    //     .run()
    //     .expect("protoc");

    // protobuf_codegen_pure::Args::new()
    //     .out_dir("src/protos")
    //     .inputs(&["protos/libraft-mem.proto"])
    //     .include("protos")
    //     .run()
    //     .expect("protoc");

    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/protos",
        input: &["protos/libraft-mem.proto"],
        includes: &["protos"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    })
    .expect("protoc");
}
