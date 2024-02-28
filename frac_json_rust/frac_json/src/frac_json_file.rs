use serde_json::Value;
use zstd::bulk::{compress, decompress};

use crate::{
    byte_stream::ByteStream,
    config::Config,
    json_types::value::{read_value, write_value},
    keys_table::{KeysTable, KeysTableLocation, KeysTables},
};

pub fn encode_frac_json(
    json: &Value,
    global_keys_table_bytes: Option<&Vec<u8>>,
    compression_level: Option<i32>,
) -> Result<Vec<u8>, String> {
    let mut header_bytes = ByteStream::new();
    let mut keys_table_bytes = ByteStream::new();
    let mut json_valuevalue_bytes = ByteStream::new();

    let global_keys_table = match global_keys_table_bytes {
        Some(bytes) => match KeysTable::read_keys_table(
            &mut ByteStream::make(bytes),
            KeysTableLocation::Global,
        ) {
            Ok(v) => Some(v),
            Err(e) => return Err(e),
        },
        None => None,
    };
    let mut keys_table = KeysTables::make(None, global_keys_table);
    write_value(json, &mut json_valuevalue_bytes, &mut keys_table)?;

    let config = Config::make(
        keys_table.has_local_keys_table(),
        compression_level.is_some(),
    );
    config.write_header(&mut header_bytes)?;
    if config.uses_local_keys_table {
        keys_table
            .local_table
            .write_keys_table(&mut keys_table_bytes)?;
    }

    let mut file_bytes: Vec<u8> = Vec::new();
    file_bytes.extend(header_bytes.as_bytes());
    match compression_level {
        None => {
            file_bytes.extend(keys_table_bytes.as_bytes());
            file_bytes.extend(json_valuevalue_bytes.as_bytes());
        }
        Some(level) => {
            let mut bytes_to_compress = keys_table_bytes.as_bytes().to_vec();
            bytes_to_compress.extend(json_valuevalue_bytes.as_bytes());
            let compressed_bytes: Vec<u8> =
                compress(&bytes_to_compress, level).map_err(|e| e.to_string())?;
            file_bytes.extend(compressed_bytes);
        }
    }
    return Ok(file_bytes);
}

pub fn decode_frac_json(
    frac_json_bytes: &Vec<u8>,
    global_keys_table_bytes: Option<&Vec<u8>>,
) -> Result<Value, String> {
    let mut bytes = ByteStream::make(frac_json_bytes);
    let config = Config::read_header(&mut bytes)?;
    if config.is_zstd_compressed {
        let compressed_bytes = bytes.read_remaining()?;
        let buffer_size = compressed_bytes.len() * 50;
        let decompressed_bytes =
            decompress(&compressed_bytes, buffer_size).map_err(|e| e.to_string())?;
        bytes = ByteStream::make(&decompressed_bytes);
    }
    let local_table = if config.uses_local_keys_table {
        Some(KeysTable::read_keys_table(
            &mut bytes,
            KeysTableLocation::Local,
        )?)
    } else {
        None
    };
    let global_keys_table = match global_keys_table_bytes {
        Some(bytes) => match KeysTable::read_keys_table(
            &mut ByteStream::make(bytes),
            KeysTableLocation::Global,
        ) {
            Ok(v) => Some(v),
            Err(e) => return Err(e),
        },
        None => None,
    };
    let keys_table = KeysTables::make(local_table, global_keys_table);

    return read_value(&mut bytes, &keys_table);
}
