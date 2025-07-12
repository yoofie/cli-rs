/* **************************************
	File Name: GIT HANDLING
	Created: Friday July 11 2025
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

/* ********************************************************
	Imports
******************************************************** */
use std::process::Command;

/* ********************************************************
	Enums & Structures
******************************************************** */

/* ********************************************************
	Public APIs
******************************************************** */

/* ********************************************************
	Private APIs
******************************************************** */
/* 
USAGE:

	match git__get_commit_id() {
		Ok(data) => println!("GIT COMMIT: {}", data),
		Err(ee) => println!("{}", ee),
	}

	match git__get_branch_name() {
		Ok(data) => println!("GIT BRANCH: {}", data),
		Err(ee) => println!("{}", ee),
	}

	match git__get_tag_name() {
		Ok(data) => println!("GIT TAG: {}", data),
		Err(ee) => println!("{}", ee),
	}
 */


/// Get commit ID
pub fn git__get_commit_id() -> Result<String, String> {
	match Command::new("git").arg("rev-parse").arg("--short").arg("HEAD").output() {
		Ok(rslt) => {
			//dbg!(rslt);
			if !rslt.stdout.is_empty() {
				//println!("{}", String::from_utf8(rslt.stdout).unwrap());
				match String::from_utf8(rslt.stdout) {
					Ok(mut data) => {
						trim_newline(&mut data);
						Ok(data)
					}
					Err(_) => Err("ERROR: Failed to convert to string".to_string()),
				}
			} else {
				Err("ERROR: Command returned empty result".to_string())
			}
		}
		Err(ee) => {
			dbg!(ee);
			Err("ERROR: Command returned empty result".to_string())
		}
	}
}

/// Get current branch name
pub fn git__get_branch_name() -> Result<String, String> {
	match Command::new("git").arg("branch").arg("--show-current").output() {
		Ok(rslt) => {
			//dbg!(&rslt);
			if !rslt.stdout.is_empty() {
				//println!("{}", String::from_utf8(rslt.stdout).unwrap());
				match String::from_utf8(rslt.stdout) {
					Ok(mut data) => {
						trim_newline(&mut data);
						Ok(data)
					}
					Err(_) => Err("ERROR: Failed to convert to string".to_string()),
				}
			} else {
				Err("ERROR: Command returned empty result".to_string())
			}
		}
		Err(ee) => {
			dbg!(ee);
			Err("ERROR: Command returned empty result".to_string())
		}
	}
}

/// Get Git Tag name
pub fn git__get_tag_name() -> Result<String, String> {
	// describe --tags --always
	match Command::new("git")
		.arg("describe")
		.arg("--tags")
		.arg("--always")
		.output()
	{
		Ok(rslt) => {
			//dbg!(rslt);
			if !rslt.stdout.is_empty() {
				//println!("{}", String::from_utf8(rslt.stdout).unwrap());
				match String::from_utf8(rslt.stdout) {
					Ok(mut data) => {
						trim_newline(&mut data);
						Ok(data)
					}
					Err(_) => Err("ERROR: Failed to convert to string".to_string()),
				}
			} else {
				Err("ERROR: Command returned empty result".to_string())
			}
		}
		Err(ee) => {
			dbg!(ee);
			Err("ERROR: Command returned empty result".to_string())
		}
	}
}

fn trim_newline(s: &mut String) {
	if s.ends_with('\n') {
		s.pop();
		if s.ends_with('\r') {
			s.pop();
		}
	}
}
