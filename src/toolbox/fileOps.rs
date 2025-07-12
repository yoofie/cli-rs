/* **************************************
	File Name: File Operations
	Created: Saturday April 06 2024
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

use std::{
	fs::{self, read_to_string, File},
	io::{Read, Write},
	path::{Path, PathBuf},
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

/// Take a string and generate a file
pub fn stringToFile(inputString: &String, outputFile: &PathBuf) {
	match fs::File::create(outputFile) {
		Ok(mut file) => {
			if let Err(ee) = file.write_all(&inputString.clone().into_bytes()) {
				println!("FAILED TO WRITE TO FILE | {ee}");
			}
		}
		Err(ee) => {
			println!("{ee} | CREATE FILE | Failed to create file");
		}
	}
}

/// Given a file path, read the file line by line and return a Vector
pub fn read_input_file(filename: &Path) -> Vec<String> {
	let mut result = Vec::with_capacity(5000);

	for line in read_to_string(filename)
		.expect("ERROR #1 | Provided file does not exist!")
		.lines()
	{
		result.push(line.to_string())
	}

	result
}
// if let Err(e) = fs::write("help.html", help.as_bytes()) {
// 	println!("asdfsd");
// }
/* ********************************************************
	Private APIs
******************************************************** */
