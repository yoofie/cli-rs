/* **************************************
	File Name:{{project-name}} app settings
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
use clap::ArgMatches;
pub use cmd::startCmdLine;
pub mod cmd;
/* ********************************************************
	Enums & Structures
******************************************************** */
#[derive(Debug)]
pub struct appSettings {
	pub appName: String,
	pub appVersion: f32,
	pub cmdLine: ArgMatches,
}
/* ********************************************************
	Public APIs
******************************************************** */

/* ********************************************************
	Private APIs
******************************************************** */
