#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{bindgen_prelude::Buffer, Error, Status};
use serde_json::Value;

use frac_json::{self, global_table_from_json_limited, global_table_from_keys};

#[napi]
pub fn encode(
  value: Value,
  global_keys_table_bytes: Option<Buffer>,
  compression_level: Option<i32>,
) -> Result<Buffer, Error> {
  frac_json::encode(
    &value,
    global_keys_table_bytes.and_then(|bytes| Some(Vec::from(bytes))),
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
pub fn decode(
  frac_json_bytes: Buffer,
  global_keys_table_bytes: Option<Buffer>,
) -> Result<Value, Error> {
  frac_json::decode(
    Vec::from(frac_json_bytes),
    global_keys_table_bytes.and_then(|bytes| Some(Vec::from(bytes))),
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
