#[derive(Copy, Clone)]
pub enum Endianess {
    LE,
    BE,
}

mod byteset;
pub use byteset::ByteSet;
