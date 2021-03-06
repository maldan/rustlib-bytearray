use crate::{
    base_info, base_read, base_write, read_number, read_vec, write_number, write_slice, write_vec,
    Endianess,
};

pub struct ByteSlice<'a> {
    position: usize,
    pub buffer: &'a mut [u8],
    pub endianess: Endianess,
}

impl<'a> ByteSlice<'a> {
    #[inline(always)]
    pub fn new(slice: &'a mut [u8], endianess: Endianess) -> ByteSlice {
        ByteSlice {
            position: 0,
            buffer: slice,
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
