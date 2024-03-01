use super::byte_stream::ByteStream;

const CURRENT_VERSION: u8 = 0;

pub struct Config {
    pub version: u8,
    pub uses_local_keys_table: bool,
    pub is_zstd_compressed: bool,
}

const FJ_MAGIC: &[u8; 2] = b"FJ";
impl Config {
    pub fn make(uses_local_keys_table: bool, is_zstd_compressed: bool) -> Config {
        Config {
            version: CURRENT_VERSION,
            uses_local_keys_table: uses_local_keys_table,
            is_zstd_compressed: is_zstd_compressed,
        }
    }

    pub fn read_header(bytes: &mut ByteStream) -> Result<Config, String> {
        let magic = bytes.read2()?;
        if magic != *FJ_MAGIC {
            return Err(format!("Invalid magic {:?}", magic));
        }
        let config = bytes.read_u8()?;
        let config = Config {
            version: config & 0b00001111,
            uses_local_keys_table: (config & 0b00010000) != 0,
            is_zstd_compressed: (config & 0b00100000) != 0,
        };
        if config.version > CURRENT_VERSION {
            return Err(format!("Unsupported version {}", config.version));
        }
        return Ok(config);
    }

    pub fn write_header(&self, bytes: &mut ByteStream) -> Result<(), String> {
        bytes.write2(FJ_MAGIC)?;
        let mut config = self.version;
        if self.uses_local_keys_table {
            config |= 0b00010000;
        }
        if self.is_zstd_compressed {
            config |= 0b00100000;
        }
        bytes.write_u8(config)?;
        return Ok(());
    }
}
