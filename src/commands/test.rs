/* **************************************
	File Name: Test
	Created: Wednesday July 03 2024
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

use clap::ArgMatches;

/* ********************************************************
	Private APIs
******************************************************** */
pub fn cmd_test(_args: &ArgMatches) -> Result<String, String> {
	// let inputFile = args.get_one::<PathBuf>("input").unwrap();
	// let outputFilePath = args.get_one::<PathBuf>("output").unwrap();
	// let file_name = outputFilePath.file_stem().unwrap().to_string_lossy();
	Ok(format!("TEST OK"))
}
