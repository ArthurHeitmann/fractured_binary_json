#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{bindgen_prelude::Buffer, Error, Status};
use serde_json::Value;

use frac_json;

#[napi]
pub fn encode_frac_json(
  value: Value,
  global_keys_table_bytes: Option<Buffer>,
  compression_level: Option<i32>,
) -> Result<Buffer, Error> {
  frac_json::encode_frac_json(
    &value,
    global_keys_table_bytes
      .and_then(|bytes| Some(Vec::from(bytes)))
      .as_ref(),
    compression_level,
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
pub fn decode_frac_json(
  frac_json_bytes: Buffer,
  global_keys_table_bytes: Option<Buffer>,
) -> Result<Value, Error> {
  frac_json::decode_frac_json(
    &Vec::from(frac_json_bytes),
    global_keys_table_bytes
      .and_then(|bytes| Some(Vec::from(bytes)))
      .as_ref(),
  )
  .map_err(|err| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to decode frac-json: {}", err),
    )
  })
}
