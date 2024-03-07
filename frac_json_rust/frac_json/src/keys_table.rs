use crate::byte_stream::ByteStream;


pub const MAX_TABLE_SIZE: usize = 0xFFFF;

pub struct GlobalKeysTable {
    table: Vec<String>,
}

impl GlobalKeysTable {
    pub fn new(table: Vec<String>) -> Self {
        GlobalKeysTable { table }
    }

    pub fn read_keys_table(bytes: &mut ByteStream) -> Result<GlobalKeysTable, String> {
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

    pub fn push_key(&mut self, key: &String) -> usize {
        self.table.push(key.to_string());
        return self.table.len() - 1;
    }
}

struct LocalKeysTable {
    encountered_keys: Vec<String>,
}

impl LocalKeysTable {
    pub fn new(encountered_keys: Vec<String>) -> LocalKeysTable {
        LocalKeysTable { encountered_keys }
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

    pub fn find_key(&self, key: &String) -> Option<usize> {
        if self.encountered_keys.is_empty() {
            return None;
        }
        return self.encountered_keys.iter().position(|x| x == key);
    }

    pub fn push_key(&mut self, key: String) -> Result<(), String> {
        if self.encountered_keys.len() >= MAX_TABLE_SIZE {
            return Err(format!(
                "LocalKeysTable is full! {} keys",
                self.encountered_keys.len()
            ));
        }
        self.encountered_keys.push(key);
        return Ok(());
    }
}

pub struct KeysTables {
    local_table: LocalKeysTable,
    global_table: GlobalKeysTable,
}

impl KeysTables {
    pub fn make(global_table: Option<GlobalKeysTable>) -> KeysTables {
        KeysTables {
            local_table: LocalKeysTable::new(Vec::new()),
            global_table: global_table.unwrap_or_else(|| GlobalKeysTable::new(Vec::new())),
        }
    }

    pub fn lookup_global_index(&self, index: usize) -> Result<&String, String> {
        self.global_table.lookup_index(index)
    }

    pub fn lookup_local_index(&self, index: usize) -> Result<&String, String> {
        self.local_table.lookup_index(index)
    }

    pub fn find_global_index(&self, key: &String) -> Option<usize> {
        self.global_table.find_key(key)
    }

    pub fn find_local_index(&self, key: &String) -> Option<usize> {
        self.local_table.find_key(key)
    }

    pub fn on_immediate_key(&mut self, key: String) -> Result<(), String> {
        self.local_table.push_key(key)
    }
}
