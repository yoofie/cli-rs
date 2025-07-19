/* **************************************
	File Name: Test
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
use crate::{fileLog, toolbox::fileOps::read_input_file};
use clap::ArgMatches;

use std::{fs, path::PathBuf};
/* ********************************************************
	Enums & Structures
******************************************************** */

/* ********************************************************
	Public APIs
******************************************************** */

/* ********************************************************
	Private APIs
******************************************************** */
pub fn cmd_test(args: &ArgMatches) -> Result<String, String> {
	let mut outputLogString = String::new();

	/* Read the input files
		******************************************************** */
	let outputFile = args.get_one::<PathBuf>("output").unwrap();
	let inputFiles: Vec<Vec<&PathBuf>> = args.get_occurrences("input").unwrap().map(Iterator::collect).collect();
	let mut inputFileCollection: Vec<Vec<String>> = Vec::with_capacity(4);

	// Flatten the vector of vectors
	let paths = inputFiles.iter().flat_map(|s| s).cloned().collect::<Vec<&PathBuf>>();

	for item in &paths {
		match fs::canonicalize(item) {
			Ok(path) => {
				outputLogString.push_str(format!("INPUT FILE: {}", path.to_string_lossy()).as_str());
				inputFileCollection.push(read_input_file(&path));
			}
			Err(ee) => {
				outputLogString.push_str(format!("{ee} | {}", item.to_string_lossy()).as_str());
			}
		}
	}

	/* Output file setup
		******************************************************** */
	let file_name = outputFile.file_stem().unwrap().to_string_lossy();
	let outputFileName = PathBuf::from(format!(
		"{}/{}.log",
		outputFile.parent().unwrap().to_string_lossy(),
		file_name.to_string()
	));

	fileLog!(
		outputLogString,
		"\nOUTPUT FILE LOCATION: {}\n\n",
		&outputFileName.to_string_lossy()
	);

	// let inputFile = args.get_one::<PathBuf>("input").unwrap();
	// let outputFilePath = args.get_one::<PathBuf>("output").unwrap();
	// let file_name = outputFilePath.file_stem().unwrap().to_string_lossy();
	Ok(format!("TEST OK"))
}
