/* **************************************
	File Name: Display
	Created: Tuesday April 02 2024
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

/* ********************************************************
	Public APIs
******************************************************** */

/* ********************************************************
	Private APIs
******************************************************** */

use itertools::Itertools;

pub fn display_array_with_offsets_string(start_Addr: usize, input: &[u8]) -> String {
	let mut retval = String::with_capacity(input.len());
	let mut row_addr = start_Addr;
	let mut rowCounter = 0;

	//retval.push_str(&format!("\n\n\n{:#<width$}", "DISPLAY BUFFER ", width = 100));
	retval.push_str(&format!("\nSTART ADDR: 0x{:05X} | ", start_Addr));
	retval.push_str(&format!("\tLEN = {}\n", input.len()));

	for row in &input.iter().chunks(16) {
		retval.push_str(&format!("{:>8} | {:>8X} | ", rowCounter, row_addr));
		for (idx, item) in row.enumerate() {
			if idx % 4 == 0 {
				retval.push_str(" ");
			}
			retval.push_str(&format!("{:02X} ", item));
		}
		retval.push('\n');
		row_addr += 0x10;
		rowCounter += 1;
	}
	retval
}

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

pub fn display_array(input: &[u8]) {
	print!("\nLEN = {} ", input.len());
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

pub fn display_array_u32(input: &[u32]) {
	print!("\nLEN = {} ", input.len());
	for (idx, item) in input.iter().enumerate() {
		if idx % 2 == 0 {
			print!(" ");
		}
		if idx % 4 == 0 {
			println!("");
		}
		print!("{:08X} ", item);
	}
	println!("");
}

pub fn display_array_as_u32_columns(input: &[u8]) {
	print!("\nLEN = {} ", input.len());
	for (idx, item) in input.iter().enumerate() {
		if idx % 4 == 0 {
			print!(" ");
		}
		if idx % 16 == 0 {
			println!("");
		}
		print!("{:02X}", item);
	}
	println!("");
}

pub fn display_array_log(log: &mut String, input: &[u8]) {
	log.push_str(format!("\nLEN = {} ", input.len()).as_str());
	for (idx, item) in input.iter().enumerate() {
		if idx % 4 == 0 {
			log.push_str(" ");
		}
		if idx % 16 == 0 {
			log.push_str("\n");
		}

		log.push_str(format!("{:02X} ", item).as_str());
	}
	log.push_str("\n");
}

pub fn display_array_inline(input: &[u8]) {
	for (idx, item) in input.iter().enumerate() {
		if idx % 4 == 0 {
			print!(" ");
		} else if idx % 16 == 0 {
			println!(" ");
		}
		print!("{:02X} ", item);
	}
	println!(" ");
}

pub fn buff_display_array(input: &[u8]) -> String {
	let mut rslt = String::new();
	for (idx, item) in input.iter().enumerate() {
		if idx % 4 == 0 {
			rslt = rslt + format!(" ").as_str();
		}
		if idx % 16 == 0 {
			rslt = rslt + format!("\n").as_str();
		}
		rslt = rslt + format!("{:02X} ", item).as_str();
	}
	rslt = rslt + format!("").as_str();
	rslt
}

pub fn buff_array_inline(input: &[u8]) -> String {
	let mut rslt = String::new();
	for (idx, item) in input.iter().enumerate() {
		if idx % 4 == 0 {
			rslt = rslt + format!(" ").as_str();
		} else if idx % 16 == 0 {
			rslt = rslt + format!(" \n").as_str();
		}
		rslt = rslt + format!("{:02X} ", item).as_str();
	}
	rslt = rslt + format!(" \n").as_str();
	rslt
}
