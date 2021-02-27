#[macro_export]
macro_rules! test_number {
    ( $i:ident, $j:expr ) => {
        paste::item! {
            #[test]
            fn [<write _ $i $j>]() {
                let mut b = ByteSet::new($j / 8);
                b.[<write _ $i $j>](4);
                b.position = 0;
                assert_eq!(b.[<read _ $i $j>](), 4);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use bytearray::byteset::ByteSet;
    use paste;

    test_number!(u, 8);
    test_number!(u, 16);
    test_number!(u, 24);
    test_number!(u, 32);
    test_number!(u, 64);
}
