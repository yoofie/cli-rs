/* **************************************
	File Name: Test 1
	Created: Wednesday July 03 2024
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

use clap::ArgMatches;
use serde_json::json;
use std::{fs, path::PathBuf};

use crate::toolbox::formatJson;

/* ********************************************************
	Private APIs
******************************************************** */
pub fn cmd_test(args: &ArgMatches) -> Result<String, String> {
	let debugPrint = match args.get_one::<bool>("debug") {
		Some(rslt) => *rslt,
		None => false,
	};
	let inputFile = args.get_one::<PathBuf>("input").expect("Input file must be provided");
	let outputFile = args.get_one::<PathBuf>("output").expect("Output file must be provided");

	/* ********************************************************
		Print to the user
	******************************************************** */
	println!("\n{:#^50}\n", " TEST COMMAND ");
	println!("Input File: {}", inputFile.to_string_lossy());
	println!("Output File: {}", outputFile.to_string_lossy());
	if debugPrint {
		println!("DEBUG MESSAGES ON!");
	}
	println!("\n{:#<50}\n", "TEST COMMAND ");

	/* ********************************************************
		Main Stuff
	******************************************************** */

	/* ********************************************************
		End
	******************************************************** */
	let log_json = json!({
		"dummy0": format!("0x{:X}", 55),
		"dummy1": 55,
		"dummy2": "dummy_val"
	});

	let prettyJson = formatJson(&log_json);

	let json_file_name = PathBuf::from(format!(
		"{}/{}.json",
		outputFile.parent().unwrap().to_string_lossy(),
		outputFile.file_stem().unwrap().to_string_lossy()
	));
	println!("{}", json_file_name.to_string_lossy());

	/* Write the user-firendly JSON file
		******************************************************** */
	if let Err(e) = fs::write(json_file_name, prettyJson) {
		println!("Failed to write to file! {e}");
	}

	Ok(format!("TEST OK"))
}
