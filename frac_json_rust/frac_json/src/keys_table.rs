use crate::byte_stream::ByteStream;

const GLOBAL_TABLE_END: usize = 0x7FFF;
const LOCAL_TABLE_OFFSET: usize = 0x8000;
const MAX_TABLE_SIZE: usize = 0x7FFF;

#[derive(Copy, Clone, Debug)]
pub enum KeysTableLocation {
    Global,
    Local,
}

pub struct KeysTable {
    table: Vec<String>,
    location: KeysTableLocation,
    key_offset: usize,
}

impl KeysTable {
    pub fn new(table: Vec<String>, location: KeysTableLocation) -> Self {
        KeysTable {
            table,
            location: location,
            key_offset: match location {
                KeysTableLocation::Global => 0,
                KeysTableLocation::Local => LOCAL_TABLE_OFFSET,
            },
        }
    }

    pub fn read_keys_table(
        bytes: &mut ByteStream,
        location: KeysTableLocation,
    ) -> Result<KeysTable, String> {
        let config = bytes.read_u8()?;
        if config != 0 {
            return Err(format!("Unsupported keys table config {}", config));
        }
        let count = bytes.read_u16()?;
        let mut mappings: Vec<String> = Vec::new();
        mappings.reserve_exact(count.into());
        for _ in 0..count {
            mappings.push(KeysTable::read_key_mapping(bytes)?);
        }
        return Ok(KeysTable::new(mappings, location));
    }

    fn read_key_mapping(bytes: &mut ByteStream) -> Result<String, String> {
        let key_length = bytes.read_u8()?;
        return Ok(bytes.read_string(key_length.into())?);
    }

    pub fn write_keys_table(&self, bytes: &mut ByteStream) -> Result<(), String> {
        let count = self.table.len();
        if count > MAX_TABLE_SIZE {
            return Err(format!("Keys table too large! {count} keys"));
        }
        bytes.write_u8(0)?;
        bytes.write_u16(count as u16)?;
        for key in self.table.iter() {
            self.write_key_mapping(key, bytes)?;
        }
        return Ok(());
    }

    fn write_key_mapping(&self, key: &String, bytes: &mut ByteStream) -> Result<(), String> {
        if key.len() >= 0xFF {
            return Err(format!("Key '{}' too long! {}", key, key.len()));
        }
        bytes.write_u8(key.len() as u8)?;
        bytes.write_string(key)?;
        return Ok(());
    }

    pub fn lookup_index(&self, index: usize) -> Result<&String, String> {
        if index >= self.table.len() {
            return Err(format!(
                "Index {index} is not in {:?} KeysTable of size {}",
                self.location,
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

    pub fn push_key(&mut self, key: &String) -> usize {
        self.table.push(key.to_string());
        return self.table.len() - 1 + self.key_offset;
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.table.len() >= MAX_TABLE_SIZE
    }
}

pub struct KeysTables {
    pub local_table: KeysTable,
    pub global_table: KeysTable,
}

impl KeysTables {
    pub fn make(local_table: Option<KeysTable>, global_table: Option<KeysTable>) -> KeysTables {
        KeysTables {
            local_table: local_table
                .unwrap_or_else(|| KeysTable::new(Vec::new(), KeysTableLocation::Local)),
            global_table: global_table
                .unwrap_or_else(|| KeysTable::new(Vec::new(), KeysTableLocation::Global)),
        }
    }

    pub fn lookup_index(&self, index: usize) -> Result<&String, String> {
        let table: &KeysTable;
        let relative_index: usize;
        if index <= GLOBAL_TABLE_END {
            table = &self.global_table;
            relative_index = index;
        } else {
            table = &self.local_table;
            relative_index = index - LOCAL_TABLE_OFFSET;
        }
        return table.lookup_index(relative_index);
    }

    pub fn lookup_key_or_insert_locally(&mut self, key: &String) -> Result<usize, String> {
        if let Some(index) = self.global_table.find_key(key) {
            return Ok(index);
        }
        if let Some(index) = self.local_table.find_key(key) {
            return Ok(index + LOCAL_TABLE_OFFSET);
        }
        if self.local_table.is_full() {
            return Err("Local keys table is full".to_string());
        }
        let new_index = self.local_table.push_key(key);
        return Ok(new_index);
    }

    pub fn has_local_keys_table(&self) -> bool {
        !self.local_table.is_empty()
    }
}
