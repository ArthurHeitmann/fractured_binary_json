use crate::byte_stream::ByteWriter;

use super::byte_stream::ByteReader;

const CURRENT_VERSION: u8 = 0;

pub struct Config {
    pub version: u8,
    pub is_zstd_compressed: bool,
    pub uses_external_dict: bool,
}

const FJ_MAGIC: &[u8; 2] = b"FJ";
impl Config {
    pub fn make(is_zstd_compressed: bool, uses_external_dict: bool) -> Config {
        Config {
            version: CURRENT_VERSION,
            is_zstd_compressed: is_zstd_compressed,
            uses_external_dict: uses_external_dict,
        }
    }

    pub fn read_header(bytes: &mut ByteReader) -> Result<Config, String> {
        let magic = bytes.read2()?;
        if magic != *FJ_MAGIC {
            return Err(format!("Invalid magic {:?}", magic));
        }
        let config = bytes.read_u8()?;
        let config = Config {
            version: config & 0b00001111,
            is_zstd_compressed: (config & 0b00010000) != 0,
            uses_external_dict: (config & 0b00100000) != 0,
        };
        if config.version > CURRENT_VERSION {
            return Err(format!("Unsupported version {}", config.version));
        }
        return Ok(config);
    }

    pub fn write_header<W: ByteWriter>(&self, bytes: &mut W) {
        bytes.write(FJ_MAGIC);
        let mut config = self.version;
        if self.is_zstd_compressed {
            config |= 0b00010000;
        }
        if self.uses_external_dict {
            config |= 0b00100000;
        }
        bytes.write_u8(config);
    }
}
