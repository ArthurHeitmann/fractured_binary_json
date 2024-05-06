#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{bindgen_prelude::Buffer, Error, Status};
use serde_json::Value;

use frac_json::{self, global_table_from_json_limited, global_table_from_keys};

#[napi(object)]
#[derive(Default)]
pub struct EncodeOptions {
  pub global_keys_table_bytes: Option<Buffer>,
  pub compression_level: Option<i32>,
  pub zstd_dict: Option<Buffer>,
}

#[napi(object)]
#[derive(Default)]
pub struct DecodeOptions {
  pub global_keys_table_bytes: Option<Buffer>,
  pub zstd_dict: Option<Buffer>,
}

#[napi]
pub fn encode(
  value: Value,
  encode_options: Option<EncodeOptions>,
) -> Result<Buffer, Error> {
  let encode_options = encode_options.unwrap_or_default();
  let global_keys_table_bytes = buffer_to_vec(encode_options.global_keys_table_bytes);
  let compression_level = encode_options.compression_level;
  let zstd_dict = buffer_to_vec(encode_options.zstd_dict);
  frac_json::encode(
    &value,
    global_keys_table_bytes.as_ref(),
    compression_level,
    zstd_dict.as_ref(),
  )
  .map_err(|err| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to encode frac-json: {}", err),
    )
  })
  .map(|vec| Buffer::from(vec))
}

#[napi]
pub fn decode(
  frac_json_bytes: Buffer,
  decode_options: Option<DecodeOptions>,
) -> Result<Value, Error> {
  let decode_options = decode_options.unwrap_or_default();
  let global_keys_table_bytes = buffer_to_vec(decode_options.global_keys_table_bytes);
  let zstd_dict = buffer_to_vec(decode_options.zstd_dict);
  frac_json::decode(
    &Vec::from(frac_json_bytes),
    global_keys_table_bytes.as_ref(),
    zstd_dict.as_ref(),
  )
  .map_err(|err| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to decode frac-json: {}", err),
    )
  })
}

#[napi]
pub fn keys_table_from_keys(keys: Vec<String>) -> Result<Buffer, Error> {
  global_table_from_keys(keys)
    .map_err(|err| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to write keys table: {}", err),
      )
    })
    .map(|vec| Buffer::from(vec))
}

#[napi]
pub fn keys_table_from_json(
  obj: Value,
  max_count: Option<i64>,
  occurrence_cutoff: Option<i64>,
) -> Result<Buffer, Error> {
  global_table_from_json_limited(
    &obj,
    max_count.map(|v| v as usize),
    occurrence_cutoff.map(|v| v as usize),
  )
  .map_err(|err| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to create keys table: {}", err),
    )
  })
  .map(|vec| Buffer::from(vec))
}

fn buffer_to_vec(buffer: Option<Buffer>) -> Option<Vec<u8>> {
  buffer.and_then(|buffer| Some(Vec::from(buffer)))
}
