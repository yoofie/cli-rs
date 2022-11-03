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
use std::path::PathBuf;
pub mod appCfg;
/* ********************************************************
	Enums & Structures
******************************************************** */
static GLOBALCFG: OnceCell<appSettings> = OnceCell::new();

/* ********************************************************
	Public APIs
******************************************************** */
fn main() {
	startCmdLine();

	let aappVersion = &GLOBALCFG.get().unwrap().cmdLine;
	let input = aappVersion.get_one::<PathBuf>("input").unwrap();

	println!("--> {}", input.to_string_lossy());
}

/* ********************************************************
	Private APIs
******************************************************** */
