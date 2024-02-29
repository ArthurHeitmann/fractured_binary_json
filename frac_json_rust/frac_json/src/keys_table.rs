use crate::byte_stream::ByteStream;

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
        let count = bytes.read_u16()?;
        let mut mappings: Vec<String> = Vec::new();
        mappings.reserve_exact(count.into());
        for i in 0..count {
            mappings.push(KeysTable::read_key_mapping(bytes, i)?);
        }
        return Ok(KeysTable::new(mappings, location));
    }

    fn read_key_mapping(bytes: &mut ByteStream, expected_index: u16) -> Result<String, String> {
        let index = bytes.read_u16()?;
        let relative_index = if index <= 0x8000 {
            index
        } else {
            index - 0x8001
        };
        if relative_index != expected_index {
            return Err(format!(
                "Expected index {expected_index} got index {index} instead"
            ));
        }
        let key_length = bytes.read_u8()?;
        return Ok(bytes.read_string(key_length.into())?);
    }

    pub fn write_keys_table(&self, bytes: &mut ByteStream) -> Result<(), String> {
        let count = self.table.len();
        if count > 0x8000 {
            return Err(format!("Keys table too large! {count} keys"));
        }
        bytes.write_u16(count as u16)?;
        let offset = match self.location {
            KeysTableLocation::Global => 0,
            KeysTableLocation::Local => LOCAL_TABLE_OFFSET,
        };
        for (i, key) in self.table.iter().enumerate() {
            self.write_key_mapping((i + offset) as u16, key, bytes)?;
        }
        return Ok(());
    }

    fn write_key_mapping(
        &self,
        index: u16,
        key: &String,
        bytes: &mut ByteStream,
    ) -> Result<(), String> {
        if key.len() > 0xFF {
            return Err(format!("Key too long! {}", key.len()));
        }
        bytes.write_u16(index)?;
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
        self.table.len() >= 0x7FFF
    }
}

pub struct KeysTables {
    pub local_table: KeysTable,
    pub global_table: KeysTable,
}

const GLOBAL_TABLE_END: usize = 0x8000;
const LOCAL_TABLE_OFFSET: usize = 0x8001;
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
        let offset: usize;
        if index <= GLOBAL_TABLE_END {
            table = &self.global_table;
            offset = 0;
        } else {
            table = &self.local_table;
            offset = LOCAL_TABLE_OFFSET;
        }
        let relative_index = index - offset;
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
