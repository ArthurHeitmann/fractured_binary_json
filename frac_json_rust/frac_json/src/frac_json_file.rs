use serde_json::Value;
use zstd::bulk::{compress, Compressor, Decompressor};

use crate::{
    byte_stream::ByteReader,
    config::Config,
    json_types::value::{read_value, write_value},
    keys_table::{DecodeKeysTables, EncodeKeysTables, GlobalKeysTable},
};

pub fn encode(
    json: &Value,
    global_keys_table_bytes: Option<&Vec<u8>>,
    compression_level: Option<i32>,
    zstd_dict: Option<&Vec<u8>>,
) -> Result<Vec<u8>, String> {
    if zstd_dict.is_some() && compression_level.is_none() {
        return Err("zstd_dict is set but compression_level is not set".to_string());
    }
    let mut header_bytes = Vec::with_capacity(3);
    let mut json_value_bytes = Vec::with_capacity(1024);

    let global_keys_table = match global_keys_table_bytes {
        Some(bytes) => match GlobalKeysTable::read_keys_table(&mut ByteReader::make(bytes)) {
            Ok(v) => Some(v),
            Err(e) => return Err(e),
        },
        None => None,
    };
    let mut keys_table = EncodeKeysTables::make(Vec::new(), global_keys_table);
    write_value(json, &mut json_value_bytes, &mut keys_table)?;

    let config = Config::make(compression_level.is_some(), zstd_dict.is_some());
    config.write_header(&mut header_bytes);

    let mut file_bytes: Vec<u8> = Vec::new();
    file_bytes.extend(header_bytes);
    match compression_level {
        None => {
            file_bytes.extend(json_value_bytes);
        }
        Some(level) => {
            let compressed_bytes = match zstd_dict {
                Some(dict) => Compressor::with_dictionary(level, dict)
                    .map_err(|e| e.to_string())?
                    .compress(&json_value_bytes)
                    .map_err(|e| e.to_string())?,
                None => compress(&json_value_bytes, level).map_err(|e| e.to_string())?
            };
            file_bytes.extend(compressed_bytes);
        }
    }
    return Ok(file_bytes);
}

pub fn decode(
    frac_json_bytes: &Vec<u8>,
    global_keys_table_bytes: Option<&Vec<u8>>,
    zstd_dict: Option<&Vec<u8>>,
) -> Result<Value, String> {
    let mut bytes = ByteReader::make(frac_json_bytes);
    let config = Config::read_header(&mut bytes)?;
    if config.uses_external_dict && zstd_dict.is_none() {
        return Err("zstd_dict is required but not provided".to_string());
    }
    let decompressed_bytes: Vec<u8>;
    if config.is_zstd_compressed {
        let compressed_bytes = bytes.read_remaining()?;
        let buffer_size = compressed_bytes.len() * 25;
        let dict = if config.uses_external_dict { zstd_dict } else { None };
        decompressed_bytes = try_decompress(&compressed_bytes, buffer_size, dict, 0)?;
        bytes = ByteReader::make(&decompressed_bytes);
    }
    let global_keys_table = match global_keys_table_bytes {
        Some(bytes) => match GlobalKeysTable::read_keys_table(&mut ByteReader::make(bytes)) {
            Ok(v) => Some(v),
            Err(e) => return Err(e),
        },
        None => None,
    };
    let mut keys_table = DecodeKeysTables::make(global_keys_table);

    return read_value(&mut bytes, &mut keys_table);
}

fn try_decompress(bytes: &[u8], buffer_size: usize, dict: Option<&Vec<u8>>, attempt: usize) -> Result<Vec<u8>, String> {
    let mut decompressor = match dict {
        Some(d) => Decompressor::with_dictionary(d).map_err(|e| e.to_string())?,
        None => Decompressor::new().map_err(|e| e.to_string())?,
    };
    let result = decompressor.decompress(bytes, buffer_size);
    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            if attempt < 3 {
                return try_decompress(bytes, buffer_size * 4, dict, attempt + 1);
            }
            return Err(e.to_string());
        }
    }
}
