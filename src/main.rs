/* **************************************
	File Name: {{project-name}}
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
use appCfg::{appSettings, startCmdLine};
use once_cell::sync::OnceCell;

pub mod appCfg;
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
			// Do something
		}
		Some(("docs", matches)) => {
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
