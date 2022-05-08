#[path = "src/types.rs"]
mod types;

use std::io::Write;

use serde_cbor::ser::to_vec_packed;

use crate::types::{MimeData, MimeDataMap};

fn main() {
  // Setup this script to re-run on changes to mime_data.json
  println!("cargo:rerun-if-changed=data/mime_data.json");

  // Read the mime_data.json file
  let inp_file = std::fs::File::open("data/mime_data.json").unwrap();
  
  // Deserialize the mime_data.json file
  let raw_data: Vec<MimeData> = serde_json::from_reader(inp_file).unwrap();

  // Create a HashMap from the deserialized data
  let mime_data_map: MimeDataMap = raw_data.into_iter().map(|m| (m.mime.to_string(), m)).collect();

  // Serialize the HashMap to a CBOR file
  let packed_data = to_vec_packed(&mime_data_map).unwrap();

  // Write the CBOR file to the output directory
  let mut out_file = std::fs::File::create("data/mime_data.cbor").unwrap();
  out_file.write_all(packed_data.as_slice()).unwrap();
}