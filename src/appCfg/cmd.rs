/* **************************************
	File Name:
	Created: Wednesday November 02 2022
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
use super::appSettings;
use crate::GLOBALCFG;
use clap::{arg, value_parser, Arg, ArgAction, Command};
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
		.subcommand(Command::new("test").alias("test0").about("Dummy description"))
		.subcommand(
			Command::new("test2")
				.about("Dummy description")
				.arg(arg!(<input> "Required input *.hex file to use").value_parser(value_parser!(PathBuf)))
				.arg(
					arg!(<output> "Required output <FILENAME> that this tool will generate")
						.value_parser(value_parser!(PathBuf)),
				)
				.arg(
					Arg::new("debug")
						.long("debug")
						.short('d')
						.action(ArgAction::SetTrue)
						.required(false)
						.help("Display debug messages"),
				),
		)
		.subcommand(
			Command::new("test3")
				.about("Dummy description")
				.arg(arg!(<input> "Required input *.hex file to use").value_parser(value_parser!(PathBuf)))
				.arg(
					arg!(<output> "Required output <FILENAME> that this tool will generate")
						.value_parser(value_parser!(PathBuf)),
				)
				.arg(
					Arg::new("debug")
						.long("debug")
						.short('d')
						.action(ArgAction::SetTrue)
						.required(false)
						.help("Display debug messages"),
				),
		)
		.subcommand(
			Command::new("docs")
				.alias("dox")
				.about("Generate & display help documentation"),
		)
		.subcommand(Command::new("info").about("Meta data information"))
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
