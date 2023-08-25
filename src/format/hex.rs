#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

use core::str;

// '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
const HEX_TABLE: [u8; 16] = [
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102,
];

pub fn hex_encode<'a, const T: usize>(bytes: &'a [u8], buffer: &'a mut [u8; T]) -> &'a str {
    let mut i = 0;

    for byte in bytes {
        let s_index = (byte >> 4) as usize;
        buffer[i] = HEX_TABLE[s_index];

        let b_index = (byte & 0xF) as usize;
        buffer[i + 1] = HEX_TABLE[b_index];

        i += 2;
    }

    unsafe { str::from_utf8_unchecked(buffer) }
}

pub fn hex_decode<'a, const T: usize>(hex: impl AsRef<[u8]>, buffer: &'a mut [u8; T]) -> [u8; T] {
    let bytes = hex.as_ref();

    let len = bytes.len() / 2;

    for i in 0..len {
        let index = i * 2;

        let s = (bytes[index] << 4) as u32;

        let b = (bytes[index + 1] as char).to_digit(16).unwrap();

        let byte = (s + b) as u8;

        buffer[i] = byte;
    }

    *buffer
}

#[cfg(feature = "alloc")]
pub fn hex_encode_string(bytes: &[u8]) -> String {
    let mut buffer = Vec::with_capacity(bytes.len() * 2);

    for byte in bytes {
        let s_index = (byte >> 4) as usize;
        buffer.push(HEX_TABLE[s_index]);

        let b_index = (byte & 0xF) as usize;
        buffer.push(HEX_TABLE[b_index]);
    }

    unsafe { String::from_utf8_unchecked(buffer) }
}

#[cfg(feature = "alloc")]
pub fn hex_decode_vec(hex: impl Into<String>) -> Vec<u8> {
    let string = hex.into();

    let bytes = string.as_bytes();

    let len = bytes.len() / 2;

    let mut buffer = Vec::with_capacity(len);

    for i in 0..len {
        let index = i * 2;
        let s = (bytes[index] << 4) as u32;

        let b = (bytes[index + 1] as char).to_digit(16).unwrap();

        let byte = (s + b) as u8;

        buffer.push(byte);
    }

    buffer
}
