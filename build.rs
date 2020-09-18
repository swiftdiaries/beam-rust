extern crate prost_build;

fn main() {
    //Client SDK according to Runner API
    prost_build::compile_protos(&["beam-v2.24.0/model/pipeline/src/main/proto/beam_runner_api.proto"],
    &["beam-v2.24.0/model/pipeline/src/main/proto/"]).unwrap();
}