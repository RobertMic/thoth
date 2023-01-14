use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    /**
     * 0001 0010 0011 0100 0101
     *
     *
     * 128  64  32  16  8  4  2   1
     *  0    0    0   0   0  0  0  0
     *
     * 0000 0001
     * 1111 1110
     *
     */
    // let num = deserialize_i8([buf[0]]);
    let num = deserialize_i8([255]);
    let num2 = deserialize_i16([1, 2]);
    let num3 = deserialize_i32([1, 2, 3, 4]);
    let num4 = deserialize_i64([1, 2, 3, 4, 5, 6, 7, 8]);
    let num5 = deserialize_u32([1, 2, 3, 4]);
    println!("{}, {}, {}, {}, {}", num, num2, num3, num4, num5);
}

fn deserialize_i8(thingy: [u8; 1]) -> i8 {
    return i8::from_be_bytes(thingy);
}

fn deserialize_i16(thingy: [u8; 2]) -> i16 {
    return i16::from_be_bytes(thingy);
}

fn deserialize_i32(thingy: [u8; 4]) -> i32 {
    return i32::from_be_bytes(thingy);
}

fn deserialize_u32(thingy: [u8; 4]) -> u32 {
    return u32::from_be_bytes(thingy);
}

fn deserialize_i64(thingy: [u8; 8]) -> i64 {
    return i64::from_be_bytes(thingy);
}
