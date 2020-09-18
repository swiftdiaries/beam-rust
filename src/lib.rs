use std::io::Cursor;
use prost::Message;

pub mod Pipeline {
    include!(env!("OUT_DIR"), "/beam_runner_api.rs");
}