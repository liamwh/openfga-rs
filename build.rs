use prost_wkt_build::*;
use std::{env, io::Result, path::PathBuf};

fn main() -> Result<()> {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_file = out.join("descriptors.bin");

    let mut prost_build = prost_build::Config::new();
    prost_build
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Struct", "::prost_wkt_types::Struct")
        .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
        .extern_path(".google.protobuf.ListValue", "::prost_wkt_types::ListValue")
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .file_descriptor_set_path(&descriptor_file);

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_with_config(
            prost_build,
            &[
                "proto/openfga/v1/authzmodel.proto",
                "proto/openfga/v1/errors_ignore.proto",
                "proto/openfga/v1/openapi.proto",
                "proto/openfga/v1/openfga.proto",
                "proto/openfga/v1/openfga_service.proto",
            ],
            &["proto/"],
        )
        .expect("failed to compile protos");

    let descriptor_bytes = std::fs::read(descriptor_file).expect("failed to read descriptor file");
    let descriptor =
        FileDescriptorSet::decode(&descriptor_bytes[..]).expect("failed to decode descriptor file");
    prost_wkt_build::add_serde(out, descriptor);

    Ok(())
}
