/* **************************************
	File Name: LOGGER STUFF
	Created: Wednesday April 23 2025
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

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

#[macro_export]
macro_rules! loggr {
	($($msg:tt)*) => {
		let loggr = unsafe { G_FOREIGN_LOGGER.get_mut().unwrap().as_mut().unwrap() };
		loggr.log(&mut format!($($msg)*));
	};
	//https://users.rust-lang.org/t/macro-with-format-as-input-argument/83596

}

/// loggr2!(outputLogString, "YASSSSS");
/// loggr2!(outputLogString, "\n\n{:=<width$}", " SUPER TEST MACRO ", width = 100);
/// loggr2!(outputLogString, "\n\n{} || {}", " SUPER TEST MACRO ", format!("wowza "));
#[macro_export]
macro_rules! fileLog {
	($logg:ident ,$($msg:tt)*) => {
		$logg.push_str(format!($($msg)*).as_str());
	};
	//https://users.rust-lang.org/t/macro-with-format-as-input-argument/83596

}

// loggr!("[PREFIX]", "FFI LOGGER CALL!!!!!");
// loggr!("FFI LOGGER CALL!!!!!");
// loggr!("[PREFIX2]", "...", "FFI LOGGER CALL!!!!!");
//

// #[macro_export]
// macro_rules! loggr {
// 	() => {};
// 	/* ($($prfix:tt)?, $($prfix2:tt)?, $($msg:tt)*) => {
// 		let loggr = unsafe { G_FOREIGN_LOGGER.get_mut().unwrap().as_mut().unwrap() };
// 		let mut prefix = format!("{} {} {}", format!($($prfix)*), format!($($prfix2)*), format!($($msg)*));
// 		loggr.log(&mut prefix);
// 	};
// 	($($prfix:tt)?, $($msg:tt)*) => {
// 		let loggr = unsafe { G_FOREIGN_LOGGER.get_mut().unwrap().as_mut().unwrap() };
// 		let mut prefix = format!("{} {}", format!($($prfix)*), format!($($msg)*));
// 		loggr.log(&mut prefix);
// 	}; */
// 	($($msg:tt)*) => {
// 		outputLogString.push_str(format!($($msg)*).as_str());
// 	};
// 	//https://users.rust-lang.org/t/macro-with-format-as-input-argument/83596
// }
