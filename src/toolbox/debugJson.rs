/* **************************************
	File Name: Debug JSON
	Created: Sunday October 13 2024
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
use serde_json::json;

use super::formatJson;
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

pub trait DebugJSON {
	fn get_raw_json_value(&self) -> serde_json::Value {
		/* let mut json: Vec<serde_json::Value> = Vec::new();

		for (bk_uid, bookmarkDatax) in self.db.iter() {
			let bookm = json!({
				"uid": "example value",
			});
			json.push(bookm);
		}
		serde_json::to_value(json).expect("ERROR #44 | FAILED TO CREATE JSON") */
		let retval = json!({
			"Example Key": "Example value"
		});
		retval
	}

	fn conv_json_to_pretty_string(&self, input: serde_json::Value) -> String {
		formatJson(&input)
	}
}
