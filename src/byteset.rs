use crate::{
    add_assign_number, add_vec, base_info, base_read, base_write, read_number, read_vec,
    write_number, write_slice, write_vec, Endianess,
};
use std::ops::Add;
use std::ops::AddAssign;

/// Byte array with fixed size
pub struct ByteSet {
    position: usize,
    pub buffer: Vec<u8>,
    pub endianess: Endianess,
}

impl ByteSet {
    #[inline(always)]
    pub fn new(size: usize, endianess: Endianess) -> ByteSet {
        ByteSet {
            position: 0,
            buffer: vec![0u8; size],
            endianess,
        }
    }

    #[inline(always)]
    pub fn from_vec(vec: Vec<u8>, endianess: Endianess) -> ByteSet {
        ByteSet {
            position: 0,
            buffer: vec,
            endianess,
        }
    }

    #[inline(always)]
    pub fn write_u8(&mut self, v: u8) {
        self.buffer[self.position] = v;
        self.position += 1;
    }

    base_info!();
    base_read!();
    base_write!();
}

add_assign_number!(u8);
add_assign_number!(i8);
add_assign_number!(u16);
add_assign_number!(i16);
add_assign_number!(u32);
add_assign_number!(i32);
add_assign_number!(u64);
add_assign_number!(i64);
add_assign_number!(f32);
add_assign_number!(f64);

/*
add_assign_vec!(u8);
add_assign_vec!(i8);
add_assign_vec!(u16);
add_assign_vec!(i16);
add_assign_vec!(u32);
add_assign_vec!(i32);
add_assign_vec!(u64);
add_assign_vec!(i64);
add_assign_vec!(f32);
add_assign_vec!(f64);*/

add_vec!(u8, 1);
add_vec!(i8, 1);
add_vec!(u16, 2);
add_vec!(i16, 2);
add_vec!(u32, 4);
add_vec!(i32, 4);
add_vec!(u64, 8);
add_vec!(i64, 8);
add_vec!(f32, 4);
add_vec!(f64, 8);

// ByteSet += ByteSet
impl AddAssign<ByteSet> for ByteSet {
    #[inline(always)]
    fn add_assign(&mut self, v: ByteSet) {
        for i in 0..v.len() {
            self.write_u8(v.buffer[i]);
        }
    }
}

// ByteSet += &str
impl AddAssign<&str> for ByteSet {
    #[inline(always)]
    fn add_assign(&mut self, v: &str) {
        for _i in 0..v.len() {
            self.write_string(v);
        }
    }
}

// ByteSet + ByteSet = new ByteSet
impl Add<ByteSet> for ByteSet {
    type Output = ByteSet;

    #[inline(always)]
    fn add(self, v: ByteSet) -> ByteSet {
        let mut b = ByteSet::new(self.len() + v.len(), self.endianess);
        for i in 0..self.len() {
            b.write_u8(self.buffer[i]);
        }
        for i in 0..v.len() {
            b.write_u8(v.buffer[i]);
        }
        b.set_pos(0);
        b
    }
}

// ByteSet + &str = new ByteSet
impl Add<&str> for ByteSet {
    type Output = ByteSet;

    #[inline(always)]
    fn add(self, v: &str) -> ByteSet {
        let mut b = ByteSet::new(self.len() + v.len(), self.endianess);
        for i in 0..self.len() {
            b.write_u8(self.buffer[i]);
        }
        b.write_string(v);
        b.set_pos(0);
        b
    }
}

// ByteSet + &String = new ByteSet
impl Add<&String> for ByteSet {
    type Output = ByteSet;

    #[inline(always)]
    fn add(self, v: &String) -> ByteSet {
        let mut b = ByteSet::new(self.len() + v.len(), self.endianess);
        for i in 0..self.len() {
            b.write_u8(self.buffer[i]);
        }
        b.write_string(v);
        b.set_pos(0);
        b
    }
}
