/* **************************************
	File Name:
	Created: Wednesday November 02 2022
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

/* ********************************************************
	Imports
******************************************************** */
use super::appSettings;
use crate::GLOBALCFG;
use clap::{arg, value_parser, Command};
use std::path::PathBuf;

/* ********************************************************
	Enums & Structures
******************************************************** */
/* ********************************************************
	Public APIs
******************************************************** */
static APP_NAME: &str = "CLI APP";

/* ********************************************************
	Private APIs
******************************************************** */

pub fn startCmdLine() {
	let cmd_line = clap::Command::new(APP_NAME)
		.version("0.1")
		.author("Yoofie <yoofie@gmail.com>")
		.about(
			r#"################### CLI APP #####################
For use with various projects


################### ############################ #####################"#,
		)
		.subcommand_required(true)
		.subcommand(
			Command::new("test")
				.alias("test0")
				.about("Dummy description")
				.arg(arg!(<input> "Required input *.hex file to use").value_parser(value_parser!(PathBuf)))
				.arg(
					arg!(<output> "Required output <FILENAME> that this tool will generate")
						.value_parser(value_parser!(PathBuf)),
				),
		)
		.get_matches();

	/* ********************************************************
		app Settings
	******************************************************** */
	let app: appSettings = appSettings {
		appName: APP_NAME.to_string(),
		appVersion: 0.1,
		cmdLine: cmd_line,
	};

	GLOBALCFG.set(app).expect("Failed to init global CFG");

	/* ********************************************************
		Some messages()
	******************************************************** */
	let appVersion = GLOBALCFG.get().unwrap().appVersion;
	println!("Hello, world! {} {}", GLOBALCFG.get().unwrap().appName, appVersion);
}
