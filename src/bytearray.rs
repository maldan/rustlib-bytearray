use crate::{
    base_info, base_read, base_write, read_number, read_vec, write_number, write_slice, write_vec,
    Endianess,
};

pub struct ByteArray {
    position: usize,
    pub buffer: Vec<u8>,
    pub endianess: Endianess,
}

impl ByteArray {
    #[inline(always)]
    pub fn new(start_size: usize, endianess: Endianess) -> ByteArray {
        ByteArray {
            position: 0,
            buffer: vec![0u8; start_size],
            endianess,
        }
    }

    #[inline(always)]
    pub fn write_u8(&mut self, v: u8) {
        if self.is_end() {
            self.buffer.push(v);
        } else {
            self.buffer[self.position] = v;
        }

        self.position += 1;
    }

    base_info!();
    base_read!();
    base_write!();
}
