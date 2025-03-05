/* **************************************
	File Name: FFI TYPES
	Created: Saturday July 20 2024
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(static_mut_refs)]

/* ********************************************************
	Imports
******************************************************** */

/* ********************************************************
	Enums & Structures
******************************************************** */

use std::ffi::c_void;

/* ********************************************************
	Public APIs
******************************************************** */
#[derive(Default, Clone, Debug)]
#[repr(C)]
pub enum ffiDataType {
	#[default]
	UTF8_STRING = 0,
	ASCII_STRING,
	U8_BYTES,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ffiString {
	pub data_type: ffiDataType,
	pub ptr: *mut u8,
	pub length: usize,
	pub capacity: usize,
}

#[derive(Clone)]
#[repr(C)]
pub struct ffiBuffer {
	pub data_type: ffiDataType,
	pub ptr: *mut u8,
	pub length: usize,
	pub capacity: usize,
}

unsafe impl Send for ffiString {}
unsafe impl Send for ffiBuffer {}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct voidPtr(*mut c_void);
pub type Callback = unsafe extern "C" fn(data: ffiString, userDataPayload: voidPtr) -> bool;

#[repr(C)]
pub struct userDataVoidPtr {
	userData: *mut c_void,
}
unsafe impl Send for userDataVoidPtr {}

/* ********************************************************
	FFI Slice Types
******************************************************** */

#[derive(Clone, Debug)]
#[repr(C)]
pub struct f32_slice {
	pub ptr: *const f32,
	pub length: usize,
	pub capacity: usize,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct u64_slice {
	pub ptr: *mut u64,
	pub length: usize,
	pub capacity: usize,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct u32_slice {
	pub ptr: *mut u32,
	pub length: usize,
	pub capacity: usize,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct u16_slice {
	pub ptr: *mut u16,
	pub length: usize,
	pub capacity: usize,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct u8_slice {
	pub ptr: *mut u8,
	pub length: usize,
	pub capacity: usize,
}

/* ********************************************************
	Private APIs
******************************************************** */
