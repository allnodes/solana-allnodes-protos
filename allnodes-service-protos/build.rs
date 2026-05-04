use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const PROTO_DIR: &str = "./proto";

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let proto_file = format!("{PROTO_DIR}/allnodes.proto");
    let oracle_proto_file = format!("{PROTO_DIR}/oracle.proto");

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("allnodes_descriptor.bin"))
        .compile(&[&proto_file, &oracle_proto_file], &[PROTO_DIR])?;

    // Re-run the build script if the proto file changes
    println!("cargo:rerun-if-changed={proto_file}");
    println!("cargo:rerun-if-changed={oracle_proto_file}");

    Ok(())
}
