#[macro_export]
macro_rules! write_number {
    ( $type:ident ) => {
        paste::item! {
            #[inline(always)]
            pub fn [<write_ $type>](&mut self, v: $type) {
                let bytes = match self.endianess {
                    Endianess::LE => v.to_le_bytes(),
                    Endianess::BE => v.to_be_bytes(),
                };
                for i in 0..bytes.len() {
                    self.write_u8(bytes[i]);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! read_number {
    ( $type:ident, $( $x:ident ),* ) => {
        paste::item! {
            #[inline(always)]
            pub fn [<read_ $type>](&mut self) -> $type {
                match self.endianess {
                    Endianess::LE => $type::from_le_bytes([
                        $(
                            self.[<read_ $x>](),
                        )*
                    ]),
                    Endianess::BE => $type::from_be_bytes([
                        $(
                            self.[<read_ $x>](),
                        )*
                    ]),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! write_vec {
    ( $type:ident ) => {
        paste::item! {
            #[inline(always)]
            pub fn [<write_ $type _vec>](&mut self, v: &Vec<$type>) {
                for i in 0..v.len() {
                    self.[<write_ $type>](v[i]);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! read_vec {
    ( $type:ident ) => {
        paste::item! {
            #[inline(always)]
            pub fn [<read_ $type _vec>](&mut self, len: usize) -> Vec<$type> {
                let mut s = vec![0 as $type; len];
                for i in 0..len {
                    s[i] = self.[<read_ $type>]();
                }
                s
            }
        }
    };
}

#[macro_export]
macro_rules! write_slice {
    ( $type:ident ) => {
        paste::item! {
            #[inline(always)]
            pub fn [<write_ $type _slice>](&mut self, s: &[$type]) {
                for i in 0..s.len() {
                    self.[<write_ $type>](s[i]);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! add_assign_number {
    ( $type:ident ) => {
        paste::item! {
            impl AddAssign<$type> for ByteSet {
                #[inline(always)]
                fn add_assign(&mut self, v: $type) {
                    self.[<write_ $type>](v);
                }
            }
        }
    };
}

/*macro_rules! add_assign_vec {
    ( $type:ident ) => {
        paste::item! {
            impl AddAssign<Vec<$type>> for ByteSet {
                #[inline(always)]
                fn add_assign(&mut self, v: Vec<$type>) {
                    self.[<write_ $type _vec>](v);
                }
            }
        }
    };
}*/

#[macro_export]
macro_rules! add_vec {
    ( $type:ident, $byte_len:stmt ) => {
        paste::item! {
            impl Add<Vec<$type>> for ByteSet {
                type Output = ByteSet;

                #[inline(always)]
                fn add(self, v: Vec<$type>) -> ByteSet {
                    let mut b = ByteSet::new(self.len() + v.len() * $byte_len, self.endianess);
                    for i in 0..self.len() {
                        b.write_u8(self.buffer[i]);
                    }
                    for i in 0..v.len() {
                        b.[<write_ $type>](v[i]);
                    }
                    b.set_pos(0);
                    b
                }
            }
        }
    };
}

#[macro_export]
macro_rules! base_info {
    () => {
        #[inline(always)]
        pub fn len(&self) -> usize {
            self.buffer.len()
        }

        #[inline(always)]
        pub fn set_pos(&mut self, p: usize) {
            self.position = p;
        }

        #[inline(always)]
        pub fn pos(&self) -> usize {
            self.position
        }

        #[inline(always)]
        pub fn is_end(&self) -> bool {
            self.position >= self.buffer.len()
        }

        #[inline(always)]
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
    };
}

#[macro_export]
macro_rules! base_read {
    () => {
        #[inline(always)]
        pub fn read_u8(&mut self) -> u8 {
            self.position += 1;
            self.buffer[self.position - 1]
        }

        #[inline(always)]
        pub fn read_i8(&mut self) -> i8 {
            self.read_u8() as i8
        }

        #[inline(always)]
        pub fn read_u24(&mut self) -> u32 {
            match self.endianess {
                Endianess::LE => {
                    (self.read_u8() as u32)
                        + (self.read_u8() as u32 * 256)
                        + (self.read_u8() as u32 * 65536)
                }
                Endianess::BE => {
                    (self.read_u8() as u32 * 65536)
                        + (self.read_u8() as u32 * 256)
                        + self.read_u8() as u32
                }
            }
        }

        #[inline(always)]
        pub fn read_string(&mut self, len: usize) -> String {
            let v = &self.buffer[self.position..(self.position + len)];
            self.position += len;
            match String::from_utf8(v.to_vec()) {
                Ok(x) => x,
                _ => String::from(""),
            }
        }

        read_number!(u16, u8, u8);
        read_number!(i16, u8, u8);
        read_number!(u32, u8, u8, u8, u8);
        read_number!(i32, u8, u8, u8, u8);
        read_number!(u64, u8, u8, u8, u8, u8, u8, u8, u8);
        read_number!(i64, u8, u8, u8, u8, u8, u8, u8, u8);
        read_number!(f32, u8, u8, u8, u8);
        read_number!(f64, u8, u8, u8, u8, u8, u8, u8, u8);

        read_vec!(u8);
        read_vec!(i8);
        read_vec!(u16);
        read_vec!(i16);
        read_vec!(u32);
        read_vec!(i32);
        read_vec!(u64);
        read_vec!(i64);
        read_vec!(f32);
        read_vec!(f64);
    };
}

#[macro_export]
macro_rules! base_write {
    () => {
        #[inline(always)]
        pub fn write_i8(&mut self, v: i8) {
            self.write_u8(v as u8);
        }

        #[inline(always)]
        pub fn write_u24(&mut self, v: u32) {
            let bytes = match self.endianess {
                Endianess::LE => v.to_le_bytes(),
                Endianess::BE => v.to_be_bytes(),
            };
            for i in 1..4 {
                self.write_u8(bytes[i]);
            }
        }

        #[inline(always)]
        pub fn write_string(&mut self, v: &str) {
            let bytes = v.as_bytes();
            for i in 0..bytes.len() {
                self.write_u8(bytes[i]);
            }
        }

        write_number!(u16);
        write_number!(i16);
        write_number!(u32);
        write_number!(i32);
        write_number!(u64);
        write_number!(i64);
        write_number!(f32);
        write_number!(f64);

        write_vec!(u8);
        write_vec!(i8);
        write_vec!(u16);
        write_vec!(i16);
        write_vec!(u32);
        write_vec!(i32);
        write_vec!(u64);
        write_vec!(i64);
        write_vec!(f32);
        write_vec!(f64);

        write_slice!(u8);
    };
}
