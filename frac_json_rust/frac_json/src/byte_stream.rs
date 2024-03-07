// LE
pub struct ByteStream {
    bytes: Vec<u8>,
    pos: usize,
}

impl ByteStream {
    pub fn new() -> ByteStream {
        ByteStream {
            bytes: Vec::new(),
            pos: 0,
        }
    }

    pub fn make(items: &Vec<u8>) -> ByteStream {
        ByteStream {
            bytes: items.clone(),
            pos: 0,
        }
    }

    pub fn seek(&mut self, pos: usize) -> Result<(), String> {
        if pos > self.bytes.len() {
            return Err(format!(
                "Cannot seek to position {}! Only {} items in ByteStream",
                pos,
                self.bytes.len()
            ));
        }
        self.pos = pos;
        return Ok(());
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    fn check_read_will_error(&self, count: usize) -> Result<(), String> {
        if self.pos + count > self.bytes.len() {
            let error_str = format!(
                "Cannot read {} items from ByteStream! Only {} items left",
                count,
                self.bytes.len()
            );
            return Err(error_str);
        }
        return Ok(());
    }

    pub fn read(&mut self, count: usize) -> Result<&[u8], String> {
        self.check_read_will_error(count)?;
        let slice = &self.bytes[self.pos..self.pos + count];
        self.pos += count;
        return Ok(slice);
    }

    pub fn read_remaining(&mut self) -> Result<&[u8], String> {
        let slice = &self.bytes[self.pos..];
        self.pos = self.bytes.len();
        return Ok(slice);
    }

    pub fn read1(&mut self) -> Result<[u8; 1], String> {
        self.check_read_will_error(1)?;
        let slice = [self.bytes[self.pos]];
        self.pos += 1;
        return Ok(slice);
    }

    pub fn read2(&mut self) -> Result<[u8; 2], String> {
        self.check_read_will_error(2)?;
        let slice = [self.bytes[self.pos], self.bytes[self.pos + 1]];
        self.pos += 2;
        return Ok(slice);
    }

    pub fn read4(&mut self) -> Result<[u8; 4], String> {
        self.check_read_will_error(4)?;
        let slice = [
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
        ];
        self.pos += 4;
        return Ok(slice);
    }

    pub fn read8(&mut self) -> Result<[u8; 8], String> {
        self.check_read_will_error(8)?;
        let slice = [
            self.bytes[self.pos + 0],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
            self.bytes[self.pos + 4],
            self.bytes[self.pos + 5],
            self.bytes[self.pos + 6],
            self.bytes[self.pos + 7],
        ];
        self.pos += 8;
        return Ok(slice);
    }

    pub fn read_u8(&mut self) -> Result<u8, String> {
        return Ok(u8::from_le_bytes(self.read1()?));
    }

    pub fn read_i8(&mut self) -> Result<i8, String> {
        return Ok(i8::from_le_bytes(self.read1()?));
    }

    pub fn read_u16(&mut self) -> Result<u16, String> {
        return Ok(u16::from_le_bytes(self.read2()?));
    }

    pub fn read_i16(&mut self) -> Result<i16, String> {
        return Ok(i16::from_le_bytes(self.read2()?));
    }

    pub fn read_u32(&mut self) -> Result<u32, String> {
        return Ok(u32::from_le_bytes(self.read4()?));
    }

    pub fn read_i32(&mut self) -> Result<i32, String> {
        return Ok(i32::from_le_bytes(self.read4()?));
    }

    pub fn read_u64(&mut self) -> Result<u64, String> {
        return Ok(u64::from_le_bytes(self.read8()?));
    }

    pub fn read_i64(&mut self) -> Result<i64, String> {
        return Ok(i64::from_le_bytes(self.read8()?));
    }

    pub fn read_f32(&mut self) -> Result<f32, String> {
        return Ok(f32::from_le_bytes(self.read4()?));
    }

    pub fn read_f64(&mut self) -> Result<f64, String> {
        return Ok(f64::from_le_bytes(self.read8()?));
    }

    pub fn read_string(&mut self, count: usize) -> Result<String, String> {
        let slice = self.read(count)?;
        let string = String::from_utf8(slice.to_vec());
        return string.map_err(|e| e.to_string());
    }

    pub fn as_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    fn check_is_at_end(&self) -> Result<(), String> {
        if self.bytes.len() != self.pos {
            return Err(format!(
                "Byte stream is not at end! pos: {}, end: {}",
                self.pos,
                self.bytes.len()
            ));
        }
        return Ok(());
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.check_is_at_end()?;
        self.bytes.extend(bytes);
        self.pos += bytes.len();
        return Ok(());
    }

    pub fn write2(&mut self, value: &[u8; 2]) -> Result<(), String> {
        self.write(value)?;
        return Ok(());
    }

    pub fn write4(&mut self, value: &[u8; 4]) -> Result<(), String> {
        self.write(value)?;
        return Ok(());
    }

    pub fn write_u8(&mut self, value: u8) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write(&bytes)?;
        return Ok(());
    }

    pub fn write_i8(&mut self, value: i8) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write(&bytes)?;
        return Ok(());
    }

    pub fn write_u16(&mut self, value: u16) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write(&bytes)?;
        return Ok(());
    }

    pub fn write_i16(&mut self, value: i16) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write(&bytes)?;
        return Ok(());
    }

    pub fn write_u32(&mut self, value: u32) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write(&bytes)?;
        return Ok(());
    }

    pub fn write_i32(&mut self, value: i32) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write(&bytes)?;
        return Ok(());
    }

    pub fn write_u64(&mut self, value: u64) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write(&bytes)?;
        return Ok(());
    }

    pub fn write_i64(&mut self, value: i64) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write(&bytes)?;
        return Ok(());
    }

    pub fn write_f32(&mut self, value: f32) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write(&bytes)?;
        return Ok(());
    }

    pub fn write_f64(&mut self, value: f64) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write(&bytes)?;
        return Ok(());
    }

    pub fn write_string(&mut self, value: &str) -> Result<(), String> {
        self.write(value.as_bytes())?;
        return Ok(());
    }
}
