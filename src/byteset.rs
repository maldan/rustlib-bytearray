use std::ops::AddAssign;

/// Byte array with fixed size
pub struct ByteSet {
    pub position: usize,
    pub buffer: Vec<u8>,
}

impl ByteSet {
    #[inline(always)]
    pub fn new(v: usize) -> ByteSet {
        ByteSet {
            position: 0,
            buffer: vec![0_u8; v],
        }
    }

    /** READ UNSIGNED */

    #[inline(always)]
    /// Read 1 unsigned byte
    pub fn read_u8(&mut self) -> u8 {
        self.position += 1;
        self.buffer[self.position - 1]
    }

    #[inline(always)]
    /// Read 2 unsigned byte
    pub fn read_u16(&mut self) -> u16 {
        self.read_u8() as u16 * 256 + self.read_u8() as u16
    }

    #[inline(always)]
    /// Read 3 unsigned byte
    pub fn read_u24(&mut self) -> u32 {
        (self.read_u8() as u32 * 65536) + (self.read_u8() as u32 * 256) + self.read_u8() as u32
    }

    #[inline(always)]
    /// Read 4 unsigned byte
    pub fn read_u32(&mut self) -> u32 {
        self.read_u16() as u32 * 65536 + self.read_u16() as u32
    }

    #[inline(always)]
    /// Read 8 unsigned byte
    pub fn read_u64(&mut self) -> u64 {
        (self.read_u32() as u64) * 4294967296 + (self.read_u32() as u64)
    }

    /** READ SIGNED */

    #[inline(always)]
    /// Read 1 signed byte
    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }

    #[inline(always)]
    /// Read 2 signed bytes
    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }

    #[inline(always)]
    /// Read 4 signed bytes
    pub fn read_i32(&mut self) -> i32 {
        self.read_u32() as i32
    }

    #[inline(always)]
    /// Read 8 signed bytes
    pub fn read_i64(&mut self) -> i64 {
        self.read_u64() as i64
    }

    /** READ FLOAT */

    #[inline(always)]
    /// Read float32 with 4 bytes precision
    pub fn read_f32(&mut self) -> f32 {
        f32::from_be_bytes([
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
        ])
    }

    #[inline(always)]
    /// Read float64 with 8 bytes precision
    pub fn read_f64(&mut self) -> f64 {
        f64::from_be_bytes([
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
            self.read_u8(),
        ])
    }

    /** READ STRING */

    #[inline(always)]
    pub fn read_string(&mut self, len: usize) -> String {
        let v = &self.buffer[self.position..(self.position + len)];
        self.position += len;
        match String::from_utf8(v.to_vec()) {
            Ok(x) => x,
            _ => panic!("Sas"),
        }
    }

    /** READ ARRAY */

    pub fn read_u8_vec(&mut self, len: usize) -> Vec<u8> {
        let s = &self.buffer[self.position..(self.position + len)];
        self.position += len;
        s.to_vec()
    }

    pub fn read_u16_vec(&mut self, len: usize) -> Vec<u16> {
        let mut s = vec![0_u16; len];
        for i in 0..len {
            s[i] = self.read_u16();
        }
        s
    }

    pub fn read_u32_vec(&mut self, len: usize) -> Vec<u32> {
        let mut s = vec![0_u32; len];
        for i in 0..len {
            s[i] = self.read_u32();
        }
        s
    }

    pub fn read_u64_vec(&mut self, len: usize) -> Vec<u64> {
        let mut s = vec![0_u64; len];
        for i in 0..len {
            s[i] = self.read_u64();
        }
        s
    }

    /** WRITE UNSIGNED */

    #[inline(always)]
    /// Write 1 unsigned byte
    pub fn write_u8(&mut self, v: u8) {
        self.buffer[self.position] = v;
        self.position += 1;
    }

    #[inline(always)]
    /// Write 2 unsigned bytes
    pub fn write_u16(&mut self, v: u16) {
        let bytes = v.to_be_bytes();
        for i in 0..2 {
            self.write_u8(bytes[i]);
        }
    }

    #[inline(always)]
    /// Write 3 unsigned bytes
    pub fn write_u24(&mut self, v: u32) {
        let bytes = v.to_be_bytes();
        for i in 1..4 {
            self.write_u8(bytes[i]);
        }
    }

    #[inline(always)]
    /// Write 4 unsigned bytes
    pub fn write_u32(&mut self, v: u32) {
        let bytes = v.to_be_bytes();
        for i in 0..4 {
            self.write_u8(bytes[i]);
        }
    }

    #[inline(always)]
    /// Write 8 unsigned bytes
    pub fn write_u64(&mut self, v: u64) {
        let bytes = v.to_be_bytes();
        for i in 0..8 {
            self.write_u8(bytes[i]);
        }
    }

    /** WRITE SIGNED */

    #[inline(always)]
    /// Write 1 signed byte
    pub fn write_i8(&mut self, v: i8) {
        self.write_u8(v as u8);
    }

    #[inline(always)]
    /// Write 2 unsigned bytes
    pub fn write_i16(&mut self, v: u16) {
        self.write_u16(v as u16);
    }
    #[inline(always)]
    /// Write 4 signed bytes
    pub fn write_i32(&mut self, v: i32) {
        self.write_u32(v as u32);
    }

    #[inline(always)]
    /// Write 8 nsigned bytes
    pub fn write_i64(&mut self, v: i64) {
        self.write_u64(v as u64);
    }

    /** WRITE FLOAT */

    #[inline(always)]
    /// Write float32 with 4 byte precision
    pub fn write_f32(&mut self, v: f32) {
        let bytes = v.to_be_bytes();
        for i in 0..4 {
            self.write_u8(bytes[i]);
        }
    }

    #[inline(always)]
    /// Write float64 with 8 byte precision
    pub fn write_f64(&mut self, v: f64) {
        let bytes = v.to_be_bytes();
        for i in 0..8 {
            self.write_u8(bytes[i]);
        }
    }

    /** WRITE STRING */
    #[inline(always)]
    pub fn write_str(&mut self, v: &str) {
        let bytes = v.as_bytes();
        for i in 0..bytes.len() {
            self.write_u8(bytes[i]);
        }
    }

    /** WRITE ARRAY */

    #[inline(always)]
    pub fn write_u8_vec(&mut self, v: Vec<u8>) {
        for i in 0..v.len() {
            self.write_u8(v[i]);
        }
    }

    #[inline(always)]
    pub fn write_u16_vec(&mut self, v: Vec<u16>) {
        for i in 0..v.len() {
            self.write_u16(v[i]);
        }
    }

    #[inline(always)]
    pub fn write_u32_vec(&mut self, v: Vec<u32>) {
        for i in 0..v.len() {
            self.write_u32(v[i]);
        }
    }

    #[inline(always)]
    pub fn write_u64_vec(&mut self, v: Vec<u64>) {
        for i in 0..v.len() {
            self.write_u64(v[i]);
        }
    }

    #[inline(always)]
    /// Return buffer length
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn print(&self) {
        let mut x = 0;

        for i in 0..self.len() {
            if x % 16 == 0 {
                print!("{:04} | ", i);
            }

            if i == self.position {
                print!("\u{001b}[41m{:02X?}\u{001b}[0m ", self.buffer[i]);
            } else {
                print!("{:02X?} ", self.buffer[i]);
            }
            if (x + 1) % 16 == 0 {
                println!("|");
            }
            x += 1;
        }
    }
}

impl AddAssign<u8> for ByteSet {
    #[inline(always)]
    fn add_assign(&mut self, v: u8) {
        self.write_u8(v);
    }
}
