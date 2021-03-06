#[derive(Copy, Clone)]
pub enum Endianess {
    LE,
    BE,
}

mod bytearray;
mod byteset;
mod byteslice;
mod r#macro;
pub use crate::bytearray::ByteArray;
pub use crate::byteset::ByteSet;
pub use crate::byteslice::ByteSlice;
