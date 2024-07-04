/* **************************************
	File Name: Display
	Created: Tuesday April 02 2024
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

/* ********************************************************
	Public APIs
******************************************************** */

/* ********************************************************
	Private APIs
******************************************************** */

pub fn display_array_subset(input: &[u8]) {
	if input.len() > 25 {
		for item in input[..25].iter() {
			print!("{} ", item);
		}
		print!("... ");
		for item in input[(input.len() - 3)..].iter() {
			print!("{} ", item);
		}
	} else {
		for item in input.iter() {
			print!("{} ", item);
		}
		print!("\tLEN = {}", input.len());
	}
}

/// Display a slice
pub fn display_array(input: &[u8]) {
	println!("\n\nLEN = {} ", input.len());
	for (idx, item) in input.iter().enumerate() {
		if idx % 4 == 0 {
			print!(" ");
		}
		if idx % 16 == 0 {
			println!("");
		}
		print!("{:02X} ", item);
	}
	println!("");
}
