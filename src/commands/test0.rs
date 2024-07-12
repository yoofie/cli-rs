/* **************************************
	File Name: TEST0
	Created: Friday June 28 2024
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
#[repr(C)]
#[repr(align(8))]
#[derive(Clone, Default)]
struct MetaDataDefn {
	field1: u32,
	field2: i32,
	field3: i32,
	field4: i32,
	field5: i32,
	field6: i64,
	field7: i64,
	pid: i64,
}
/* ********************************************************
	Public APIs
******************************************************** */

use clap::ArgMatches;

/* ********************************************************
	Private APIs
******************************************************** */
pub fn test0_start(_args: &ArgMatches) -> Result<String, String> {
	let mut test = MetaDataDefn::default();
	let ptr = &mut test;
	ptr.field1 = 0xffff_0000;
	// https://stackoverflow.com/questions/56613171/does-casting-pointers-in-rust-have-the-same-behavior-as-reinterpret-cast-in-c
	let field1 = unsafe { (ptr.field1 as *mut u32) };
	let vv = std::ptr::addr_of!(ptr.field1);
	println!("CNC Version: {:X}", vv as u64);
	let vv = std::ptr::addr_of!(ptr.field2);
	println!("field2: {:X}", vv as u64);
	let vv = std::ptr::addr_of!(ptr.field3);
	println!("field3: {:X}", vv as u64);
	println!(
		"CNC Version: {:X} | {:X}",
		field1 as u64,
		std::ptr::addr_of!(field1) as u64
	);

	ptr.field1 = 6;

	println!("CNC Version: {:X}", ptr.field1 as u64);

	Ok(format!("ALL OK"))
}
