# ByteArray

Library for handy works with bytes and bits.

[![License](https://img.shields.io/github/license/maldan/rustlib-bytearray)](https://github.com/maldan/rustlib-bytearray/blob/main/LICENSE)
[![hits](https://hits.deltapapa.io/github/maldan/rustlib-bytearray.svg)](https://github.com/maldan/rustlib-bytearray)
<a href="https://github.com/maldan/rustlib-bytearray/pulse" alt="Activity">
<img src="https://img.shields.io/github/commit-activity/m/maldan/rustlib-bytearray" />
</a>
![GitHub Repo stars](https://img.shields.io/github/stars/maldan/rustlib-bytearray)

## Description

There are 3 structs `ByteSet`, `ByteSlice` and `ByteArray`.

### ByteSet

It's a wrapper for `Vec<u8>`. It has fixed size at runtime. But you can write and read it. Note! If you go beyound capacity of array it will panic.

### ByteSlice

It's a wrapper for `&[u8]`. You can't allocate it directly. You can only wrap existing slice and work with it. Note! If you go beyound capacity of array it will panic.

### ByteArray

It's also wrapper for `Vec<u8>` but you can expand it at runtime. It will panic only when you read beyound capacity. Any write operation can't cause panic.

## What Can I Do?

- Write and read any `number`
- Write and read `string`
- Write and read `Vec<number>`
- Work with `LE` and `BE` endians

## Example

```rs
// Import what you need
use bytearray::{ByteArray, ByteSet, ByteSlice, Endianess};
```

```rs
// Test ByteSet
let mut b = ByteSet::new(8, Endianess::LE);
b.write_u64(123456789);
b.set_pos(0);
print!("{}", b.read_u64()); // 123456789
```

```rs
// Test write panic
let mut b = ByteSet::new(4, Endianess::LE);
b.write_u64(123456789); // panic!
```

```rs
// Test read panic
let mut b = ByteSet::new(4, Endianess::LE);
b.write_u32(1234);
b.read_u32(); // panic!
```

```rs
// Test ByteArray
let mut b = ByteArray::new(0, Endianess::LE);
b.write_u64(123456789);
b.write_u64(123456789);
b.write_u64(123456789);
println!("{}", b.len()); // 24
```

```rs
// Test ByteSlice
let mut slice = [0u8; 65536];
let mut b = ByteSlice::new(&mut slice, Endianess::LE);
b.write_u64(123456789);
b.set_pos(0);
print!("{}", b.read_u64()); // 123456789
```
