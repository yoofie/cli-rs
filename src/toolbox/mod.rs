/* **************************************
	File Name: Rust Toolbox
	Created: Wednesday March 27 2024
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use serde::Serialize;
pub mod display;
pub mod fileOps;
/* ********************************************************
	Imports
******************************************************** */

/* ********************************************************
	Enums & Structures
******************************************************** */

/* ********************************************************
	Public APIs
******************************************************** */

/* ********************************************************
	Private APIs
******************************************************** */

fn to_array<const N: usize>(s: &str) -> [char; N] {
	let mut chars = s.chars();
	[(); N].map(|_| chars.next().unwrap())
}

pub fn formatJson(input: &serde_json::Value) -> String {
	let mut buf = Vec::new();
	let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
	let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
	input.serialize(&mut ser).unwrap();
	let jsonString = String::from_utf8(buf).expect("Failed to create JSON string");
	//println!("{}", String::from_utf8(buf).unwrap());
	jsonString
}

/* ********************************************************
	Read/Write Byte manipulation
******************************************************** */

pub fn write_le_16(buf: &mut [u8], val: u16) {
	buf[..2].copy_from_slice(&val.to_le_bytes());
}
pub fn write_be_16(buf: &mut [u8], val: u16) {
	buf[..2].copy_from_slice(&val.to_be_bytes());
}
pub fn write_le_32(buf: &mut [u8], val: u32) {
	buf[..4].copy_from_slice(&val.to_le_bytes());
}
pub fn write_be_32(buf: &mut [u8], val: u32) {
	buf[..4].copy_from_slice(&val.to_be_bytes());
}

/// Byte swap elements in array
/// Before:
/// ```
/// 91 E0 01 F8   D9 FF 68 61   DC 0F 00 90
/// ```
/// After:
/// ```
/// F8 01 E0 91   61 68 FF D9   90 00 0F DC
/// ```
pub fn byteSwap_u32_array(input_vec: &mut [u8]) {
	if input_vec.len() % 4 == 0 {
		for chunk in input_vec[..].chunks_exact_mut(4) {
			chunk.swap(0, 3);
			chunk.swap(1, 2);
		}
	} else {
		println!("ERROR: byteSwap_u32_array() must be divisible by 4!");
	}
}
pub fn byteSwap_u16_array(input_vec: &mut [u8]) {
	if input_vec.len() % 2 == 0 {
		for chunk in input_vec[..].chunks_exact_mut(2) {
			chunk.swap(0, 1);
		}
	} else {
		println!("ERROR: byteSwap_u16_array() must be divisible by 2!");
	}
}

pub fn byte_slice_to_u32_be(input: &[u8]) -> u32 {
	let mut slice = &input[0..4];
	//dbg!(slice);
	let val = slice.read_u32::<BigEndian>().unwrap();
	val
}

pub fn byte_slice_to_u32_le(input: &[u8]) -> u32 {
	let mut slice = &input[0..4];
	//dbg!(slice);
	let val = slice.read_u32::<LittleEndian>().unwrap();
	val
}

pub fn byte_slice_to_u16_le(input: &[u8]) -> u16 {
	let mut slice = &input[0..2];
	//dbg!(slice);
	let val = slice.read_u16::<LittleEndian>().unwrap();
	val
}

pub fn byte_slice_to_u16_be(input: &[u8]) -> u16 {
	let mut slice = &input[0..2];
	//dbg!(slice);
	let val = slice.read_u16::<BigEndian>().unwrap();
	val
}

pub fn u32_to_array(x: u32) -> [u8; 4] {
	let b1: u8 = ((x >> 24) & 0xff) as u8;
	let b2: u8 = ((x >> 16) & 0xff) as u8;
	let b3: u8 = ((x >> 8) & 0xff) as u8;
	let b4: u8 = (x & 0xff) as u8;
	return [b1, b2, b3, b4];
}

/* ********************************************************
	Struct to array
******************************************************** */
/*
/// Convert struct into array
impl From<[u8; 0x40]> for partitionHeader_v1 {
	fn from(bytes: [u8; 0x40]) -> Self {
		unsafe { std::mem::transmute(bytes) }
	}
}

/// Convert struct into array?
impl From<partitionHeader_v1> for [u8; 0x40] {
	fn from(h: partitionHeader_v1) -> Self {
		unsafe { std::mem::transmute(h) }
	}
}

pub fn to_array(input: &[u8]) -> [u8; 0x40] {
	input.try_into().expect("slice with incorrect length")
}

pub fn construct_from_u8_slice(input: &[u8]) -> partitionHeader_v1 {
	let slice = partitionHeader_v1::to_array(&input[..64]);
	let retval = unsafe { transmute::<[u8; 0x40], partitionHeader_v1>(slice) };
	retval
}
*/
/* ********************************************************
	TESTS
******************************************************** */
#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test0() {
		println!("{:?}", to_array::<5>("Hello"));
		//assert_eq!();
	}

	#[test]
	fn test_u32_write_partition_identity_0() {

		//assert_eq!();
	}

	#[test]
	fn test_byte_slice_to_u32() {
		//let data: [u8; 4] = [1, 2, 3, 4];
		let buf: &[u8] = &[0, 0, 0, 1];

		let result = byte_slice_to_u32_be(&buf[..]);
		//replaceSlice_u32(&mut before, 0, appHdr.partitionIdentity);

		assert_eq!(result, 0x0000_0001);
	}

	#[test]
	fn test_byte_slice_to_u32_1() {
		let buf: &[u8] = &[5, 4, 3, 2, 1];

		let result = byte_slice_to_u32_be(&buf[..]);
		//let slice = &buf[0..4];
		//let num = u32::from_le_bytes(slice);

		assert_eq!(result, 0x0403_0201);
	}
	#[test]
	fn test_byte_slice_to_u32_2() {
		let buf: &[u8] = &[4, 3, 2, 1];

		// We use std rust here
		let result = u32::from_be_bytes(buf[..].try_into().unwrap());

		assert_eq!(result, 0x0403_0201);
	}

	#[test]
	fn test_byte_slice_to_u32_le_0() {
		//let data: [u8; 4] = [1, 2, 3, 4];
		let buf: &[u8] = &[0, 0, 0, 1];

		let result = byte_slice_to_u32_le(&buf[..]);
		//replaceSlice_u32(&mut before, 0, appHdr.partitionIdentity);

		assert_eq!(result, 0x1000_0000);
	}
	#[test]
	fn test_byte_swap_to_u16_le_0() {
		//let data: [u8; 4] = [1, 2, 3, 4];
		let buf: &mut [u8] = &mut [1, 2, 3, 4];

		byteSwap_u16_array(&mut buf[..]);
		//replaceSlice_u32(&mut before, 0, appHdr.partitionIdentity);

		assert_eq!(buf, &[2, 1, 4, 3]);
	}

	#[test]
	fn test_u32_to_array() {
		//let data: [u8; 4] = [1, 2, 3, 4];
		//let buf: &mut [u8] = &mut [0xaa, 0xbb, 0xcc, 0xdd];

		let rslt = u32_to_array(0xaabbccdd);
		//println!("{:X}", rslt);
		assert_eq!(&rslt, &[0xaa, 0xbb, 0xcc, 0xdd]);
	}
}
