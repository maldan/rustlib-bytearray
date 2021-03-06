#[derive(Copy, Clone)]
pub enum Endianess {
    LE,
    BE,
}

mod bytearray;
mod byteset;
mod byteslice;
mod r#macro;
pub use bytearray::ByteArray;
pub use byteset::ByteSet;
pub use byteslice::ByteSlice;
