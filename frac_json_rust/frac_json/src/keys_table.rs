use crate::byte_stream::{ByteReader, ByteWriter};

pub const MAX_TABLE_SIZE: usize = 0xFFFF;
pub const MAX_KEY_LENGTH: usize = 0xFFFF;

pub struct GlobalKeysTable {
    table: Vec<String>,
}

impl GlobalKeysTable {
    pub fn new(table: Vec<String>) -> Self {
        GlobalKeysTable { table }
    }

    pub fn read_keys_table(bytes: &mut ByteReader) -> Result<GlobalKeysTable, String> {
        let config = bytes.read_u8()?;
        if config != 0 {
            return Err(format!("Unsupported keys table config {}", config));
        }
        let count = bytes.read_u16()?;
        let mut mappings: Vec<String> = Vec::new();
        mappings.reserve_exact(count.into());
        for _ in 0..count {
            mappings.push(GlobalKeysTable::read_key_mapping(bytes)?);
        }
        return Ok(GlobalKeysTable::new(mappings));
    }

    fn read_key_mapping(bytes: &mut ByteReader) -> Result<String, String> {
        let key_length = bytes.read_u16()?;
        return Ok(bytes.read_string(key_length.into())?);
    }

    pub fn write_keys_table<W: ByteWriter>(&self, bytes: &mut W) -> Result<(), String> {
        let count = self.table.len();
        if count > MAX_TABLE_SIZE {
            return Err(format!("Keys table too large! {count} keys"));
        }
        bytes.write_u8(0);
        bytes.write_u16(count as u16);
        for key in self.table.iter() {
            self.write_key_mapping(key, bytes)?;
        }
        return Ok(());
    }

    fn write_key_mapping<W: ByteWriter>(&self, key: &String, bytes: &mut W) -> Result<(), String> {
        if key.len() > MAX_KEY_LENGTH {
            return Err(format!("Key '{}' too long! {}", key, key.len()));
        }
        bytes.write_u16(key.len() as u16);
        bytes.write_string(key);
        return Ok(());
    }

    pub fn lookup_index(&self, index: usize) -> Result<&String, String> {
        if index >= self.table.len() {
            return Err(format!(
                "Index {index} is not in GlobalKeysTable of size {}",
                self.table.len()
            ));
        }
        return Ok(&self.table[index]);
    }

    pub fn find_key(&self, key: &String) -> Option<usize> {
        if self.table.is_empty() {
            return None;
        }
        return self.table.iter().position(|x| x == key);
    }
}

struct LocalEncodeKeysTable<'a> {
    encountered_keys: Vec<&'a String>,
}

struct LocalDecodeKeysTable {
    encountered_keys: Vec<String>,
}

impl<'a> LocalEncodeKeysTable<'a> {
    pub fn new(encountered_keys: Vec<&String>) -> LocalEncodeKeysTable {
        LocalEncodeKeysTable { encountered_keys }
    }

    pub fn find_key(&self, key: &String) -> Option<usize> {
        if self.encountered_keys.is_empty() {
            return None;
        }
        return self.encountered_keys.iter().position(|x| *x == key);
    }

    pub fn push_key_ref(&mut self, key: &'a String) {
        if self.encountered_keys.len() < MAX_TABLE_SIZE {
            self.encountered_keys.push(key);
        }
    }
}

impl LocalDecodeKeysTable {
    pub fn new() -> LocalDecodeKeysTable {
        LocalDecodeKeysTable {
            encountered_keys: Vec::new(),
        }
    }

    pub fn lookup_index(&self, index: usize) -> Result<&String, String> {
        if index >= self.encountered_keys.len() {
            return Err(format!(
                "Index {index} is not in LocalKeysTable of size {}",
                self.encountered_keys.len()
            ));
        }
        return Ok(&self.encountered_keys[index]);
    }

    pub fn push_key(&mut self, key: &String) {
        if self.encountered_keys.len() < MAX_TABLE_SIZE {
            self.encountered_keys.push(key.clone());
        }
    }
}

pub struct EncodeKeysTables<'a> {
    local_table: LocalEncodeKeysTable<'a>,
    global_table: GlobalKeysTable,
}

pub struct DecodeKeysTables {
    local_table: LocalDecodeKeysTable,
    global_table: GlobalKeysTable,
}

impl<'a> EncodeKeysTables<'a> {
    pub fn make(
        local_table: Vec<&String>,
        global_table: Option<GlobalKeysTable>,
    ) -> EncodeKeysTables {
        EncodeKeysTables {
            local_table: LocalEncodeKeysTable::new(local_table),
            global_table: global_table.unwrap_or_else(|| GlobalKeysTable::new(Vec::new())),
        }
    }

    pub fn find_global_index(&self, key: &String) -> Option<usize> {
        self.global_table.find_key(key)
    }

    pub fn find_local_index(&self, key: &String) -> Option<usize> {
        self.local_table.find_key(key)
    }

    pub fn on_immediate_key<'b: 'a>(&mut self, key: &'b String) {
        self.local_table.push_key_ref(key);
    }
}

impl DecodeKeysTables {
    pub fn make(global_table: Option<GlobalKeysTable>) -> DecodeKeysTables {
        DecodeKeysTables {
            local_table: LocalDecodeKeysTable::new(),
            global_table: global_table.unwrap_or_else(|| GlobalKeysTable::new(Vec::new())),
        }
    }

    pub fn lookup_global_index(&self, index: usize) -> Result<&String, String> {
        self.global_table.lookup_index(index)
    }

    pub fn lookup_local_index(&self, index: usize) -> Result<&String, String> {
        self.local_table.lookup_index(index)
    }

    pub fn on_immediate_key(&mut self, key: &String) {
        self.local_table.push_key(key);
    }
}
