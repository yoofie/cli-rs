/* **************************************
	File Name: File Operations
	Created: Saturday April 06 2024
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

/* ********************************************************
	Imports
******************************************************** */

/* ********************************************************
	Enums & Structures
******************************************************** */

use std::{
	fs::{self, File},
	io::Read,
	path::PathBuf,
};

/* ********************************************************
	Public APIs
******************************************************** */
// Read a file as binary
pub fn get_file_as_byte_vec(filename: &PathBuf) -> Vec<u8> {
	let mut f = File::open(&filename).expect("no file found");
	let metadata = fs::metadata(&filename).expect("unable to read metadata");
	let mut buffer = vec![0; metadata.len() as usize];
	f.read(&mut buffer).expect("buffer overflow");

	buffer
}

/* ********************************************************
	Private APIs
******************************************************** */
