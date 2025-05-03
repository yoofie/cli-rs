/* **************************************
	File Name: {{project-name}}
	Created: Wednesday November 02 2022
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(static_mut_refs)]
#![allow(unexpected_cfgs)]

/* ********************************************************
Imports
******************************************************** */
// use anyhow::{anyhow, Result};
use appCfg::{appSettings, startCmdLine};
use commands::{info::appInfo, test::cmd_test, test2::cmd_test2, test3::cmd_test3};
use once_cell::sync::OnceCell;
use std::{fs, process::Command};

pub mod appCfg;
pub mod commands;
pub mod ffiToolbox;
pub mod toolbox;

/* ********************************************************
	Enums & Structures
******************************************************** */
static GLOBALCFG: OnceCell<appSettings> = OnceCell::new();

/* ********************************************************
	Public APIs
******************************************************** */
fn main() {
	startCmdLine();

	let args = &GLOBALCFG.get().unwrap().cmdLine;

	match args.subcommand() {
		Some(("test", matches)) => {
			println!("Do something!");
			if let Err(msg) = cmd_test(&matches) {
				println!("{}", msg);
			}
		}
		Some(("test2", matches)) => {
			println!("TEST 2 | Do something!");
			if let Err(msg) = cmd_test2(&matches) {
				println!("{}", msg);
			}
		}
		Some(("test3", matches)) => {
			println!("TEST 3 | Do something!");
			if let Err(msg) = cmd_test3(&matches) {
				println!("{}", msg);
			}
		}
		Some(("info", _matches)) => {
			appInfo();
		}
		Some(("docs", _matches)) => {
			let help = include_str!("./appCfg/help.html");
			if let Err(e) = fs::write("help.html", help.as_bytes()) {
				println!("ERROR  |{e}");
			}

			Command::new("explorer")
				.arg("/select,")
				.arg("help.html")
				.output()
				.expect("failed to execute process");
			opener::open("help.html").unwrap();
		}
		_ => unreachable!("clap should ensure we don't get here"),
	};

	println!("TODAY: {}", chrono::offset::Utc::now());
}

/* ********************************************************
	Private APIs
******************************************************** */
