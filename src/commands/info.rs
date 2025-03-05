/* **************************************
	File Name: Info RS
	Created: Wednesday November 13 2024
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

/* ********************************************************
	Private APIs
******************************************************** */

/* ********************************************************
	Traits
******************************************************** */
pub fn appInfo() {
	/* println!("debug:{}", shadow_rs::is_debug()); // check if this is a debug build. e.g 'true/false'
	println!("branch:{}", shadow_rs::branch()); // get current project branch. e.g 'master/develop'
	println!("tag:{}", shadow_rs::tag()); // get current project tag. e.g 'v1.3.5'
	println!("git_clean:{}", shadow_rs::git_clean()); // get current project clean. e.g 'true/false'
	println!("git_status_file:{}", shadow_rs::git_status_file()); // get current project statue file. e.g '  * examples/builtin_fn.rs (dirty)' */
	println!("----");
	//println!("VERSION           : {}", build::VERSION); //print version const
	//println!("VERSION           : {}", build::PKG_VERSION); //0.3.13
	//println!("CLAP_LONG_VERSION : {}", build::CLAP_LONG_VERSION); //print CLAP_LONG_VERSION const
	//println!("BRANCH            : {}", build::BRANCH); //master
	//println!("SHORT_COMMIT      : {}", build::SHORT_COMMIT); //8405e28e
	//println!("COMMIT_HASH       : {}", build::COMMIT_HASH); //8405e28e64080a09525a6cf1b07c22fcaf71a5c5
	//println!("COMMIT_DATE       : {}", build::COMMIT_DATE); //2021-08-04 12:34:03 +00:00
	//println!("COMMIT_AUTHOR     : {}", build::COMMIT_AUTHOR); //baoyachi
	//println!("COMMIT_EMAIL      : {}", build::COMMIT_EMAIL); //xxx@gmail.com
	//println!("----");
	//println!("BUILD_OS          : {}", build::BUILD_OS); //macos-x86_64
	//println!("RUST_VERSION      : {}", build::RUST_VERSION); //rustc 1.45.0 (5c1f21c3b 2020-07-13)
	//println!("RUST_CHANNEL      : {}", build::RUST_CHANNEL); //stable-x86_64-apple-darwin (default)
	//println!("CARGO_VERSION     : {}", build::CARGO_VERSION); //cargo 1.45.0 (744bd1fbb 2020-06-15)
	//println!("----"); //println!("{}", build::CARGO_TREE); //like command:cargo tree
	//println!("PROJECT_NAME      : {}", build::PROJECT_NAME); //shadow-rs
	//println!("BUILD_TIME        : {}", build::BUILD_TIME); //2020-08-16 14:50:25
	//println!("BUILD_RUST_CHANNEL: {}", build::BUILD_RUST_CHANNEL); //debug
}
