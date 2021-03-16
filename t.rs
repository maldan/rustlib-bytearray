use bytearray::{ByteSet, Endianess};

fn main() {
    /*let now = std::time::Instant::now();
    let mut gas = [0u8; 65536];
    let mut b = ByteSlice::new(&mut gas, Endianess::LE);
    for i in 0..65535 {
        b.write_u8(4);
    }
    println!("{:?}", now.elapsed());*/

    /*let now = std::time::Instant::now();
    let mut b = ByteSet::new(16777216, Endianess::LE);
    for i in 0..16777216 {
        b.write_u8(3);
    }
    println!("{:?}", now.elapsed());*/

    /*let mut b = ByteArray::new(0, Endianess::LE);
    b.write_u8(10);
    b.write_u8(10);
    b.write_u8(10);
    b.write_u8(10);
    b.write_u8_slice(&[0u8, 11u8]);
    b.write_string("GASON!");
    b.set_pos(4);
    println!("{:?}", b.read_i8_vec(6));
    b.print();*/

    // Test oveflow
    let mut b = ByteSet::new(4, Endianess::LE);
    b.write_u32(1234);
}
