use serde_json::Value;
use zstd::bulk::{compress, decompress};

use crate::{
    byte_stream::ByteStream,
    config::Config,
    json_types::value::{read_value, write_value},
    keys_table::{DecodeKeysTables, EncodeKeysTables, GlobalKeysTable},
};

pub fn encode_frac_json(
    json: &Value,
    global_keys_table_bytes: Option<Vec<u8>>,
    compression_level: Option<i32>,
) -> Result<Vec<u8>, String> {
    let mut header_bytes = ByteStream::make(Vec::with_capacity(3));
    let mut json_value_bytes = ByteStream::make(Vec::with_capacity(1024));

    let global_keys_table = match global_keys_table_bytes {
        Some(bytes) => match GlobalKeysTable::read_keys_table(&mut ByteStream::make(bytes)) {
            Ok(v) => Some(v),
            Err(e) => return Err(e),
        },
        None => None,
    };
    let mut keys_table = EncodeKeysTables::make(Vec::new(), global_keys_table);
    write_value(json, &mut json_value_bytes, &mut keys_table)?;

    let config = Config::make(compression_level.is_some(), false);
    config.write_header(&mut header_bytes)?;

    let mut file_bytes: Vec<u8> = Vec::new();
    file_bytes.extend(header_bytes.as_bytes());
    match compression_level {
        None => {
            file_bytes.extend(json_value_bytes.as_bytes());
        }
        Some(level) => {
            let bytes_to_compress = json_value_bytes.as_bytes();
            let compressed_bytes: Vec<u8> =
                compress(&bytes_to_compress, level).map_err(|e| e.to_string())?;
            file_bytes.extend(compressed_bytes);
        }
    }
    return Ok(file_bytes);
}

pub fn decode_frac_json(
    frac_json_bytes: Vec<u8>,
    global_keys_table_bytes: Option<Vec<u8>>,
) -> Result<Value, String> {
    let mut bytes = ByteStream::make(frac_json_bytes);
    let config = Config::read_header(&mut bytes)?;
    if config.is_zstd_compressed {
        let compressed_bytes = bytes.read_remaining()?;
        let buffer_size = compressed_bytes.len() * 50;
        let decompressed_bytes =
            decompress(&compressed_bytes, buffer_size).map_err(|e| e.to_string())?;
        bytes = ByteStream::make(decompressed_bytes);
    }
    let global_keys_table = match global_keys_table_bytes {
        Some(bytes) => match GlobalKeysTable::read_keys_table(&mut ByteStream::make(bytes)) {
            Ok(v) => Some(v),
            Err(e) => return Err(e),
        },
        None => None,
    };
    let mut keys_table = DecodeKeysTables::make(global_keys_table);

    return read_value(&mut bytes, &mut keys_table);
}
