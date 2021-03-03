#[macro_export]
macro_rules! test_number {
    ( $i:ident, $j:expr ) => {
        paste::item! {
            #[test]
            fn [<write _ $i $j>]() {
                let mut b = ByteSet::new($j / 8, Endianess::LE);
                b.[<write _ $i $j>](4);
                b.set_pos(0);
                assert_eq!(b.[<read _ $i $j>](), 4);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use bytearray::ByteSet;
    use bytearray::Endianess;
    use paste;

    test_number!(u, 8);
    test_number!(u, 16);
    test_number!(u, 32);
    test_number!(u, 64);

    #[test]
    fn from() {
        let mut b = ByteSet::from(&[1u8, 2u8, 3u8], Endianess::LE);
        assert_eq!(b.read_u8(), 1u8);
        assert_eq!(b.read_u8(), 2u8);
        assert_eq!(b.read_u8(), 3u8);
    }
}
