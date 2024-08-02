/* **************************************
	File Name: Callback System
	Created: Monday July 01 2024
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

/* ********************************************************
	Imports
******************************************************** */

use std::{collections::HashMap, fmt::Display, hash::Hash};

use super::ffiTypes::{ffiDataType, ffiString, voidPtr, Callback};

/* ********************************************************
	Enums & Structures
******************************************************** */
/* #[derive(Default, Clone, Debug)]
#[repr(C)]
pub enum ffiDataType {
	#[default]
	UTF8_STRING = 0,
	ASCII_STRING,
	U8_BYTES,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ffiString {
	pub data_type: ffiDataType,
	pub ptr: *mut u8,
	pub length: usize,
	pub capacity: usize,
}
 */
//pub type voidPtr = *mut c_void;
/* #[derive(Clone, Debug)]
pub struct voidPtr(*mut c_void); */
//pub type Callback = unsafe extern "C" fn(data: ffiString, userDataPayload: voidPtr) -> bool;
unsafe impl Send for callbackDatax {}
unsafe impl Sync for callbackDatax {}
//unsafe impl Send for callbackDBB<quicServerEvent> {}

#[derive(Clone, Debug)]
#[repr(C)]

pub struct callbackDatax {
	pub userCallback: Callback,
	pub userDataPayload: voidPtr,
}
/* ********************************************************
	Public APIs
******************************************************** */

/* ********************************************************
	Private APIs
******************************************************** */

#[derive(Debug, Clone)]
//pub struct callbackDB(HashMap<opCode, Option<Callback>>);
pub struct callbackDBB<T> {
	pub db: HashMap<T, Option<callbackDatax>>,
}
/* ********************************************************
	Private APIs
******************************************************** */
impl<T> callbackDBB<T>
where
	T: Display + Clone + Eq + Hash,
{
	pub fn new() -> callbackDBB<T> {
		let hashmapp: HashMap<T, Option<callbackDatax>> = HashMap::with_capacity(10);
		callbackDBB { db: hashmapp }
	}

	pub fn execute_callback(&self, op: T) -> Result<(), String> {
		match self.db.get(&op) {
			Some(Some(callbackData)) => {
				let ffi = ffiString {
					data_type: ffiDataType::ASCII_STRING,
					ptr: std::ptr::null_mut(),
					length: 0,
					capacity: 16,
				};
				if unsafe { !(callbackData.userCallback)(ffi, callbackData.userDataPayload.clone()) } {
					println!("FFI Call {} returned failure!", op);
				};
				Ok(())
			}
			Some(None) => Err(format!("#45 | CMD \"{}\" not registered in callback list", op)),
			None => Err(format!("#44 | CMD \"{}\" not registered in callback list", op)),
		}
	}

	pub fn execute_callback_with_data(&self, msg: &mut String, op: T) -> Result<(), String> {
		match self.db.get(&op) {
			Some(Some(callbackData)) => {
				let ffi = ffiString {
					data_type: ffiDataType::UTF8_STRING,
					ptr: msg.as_mut_ptr(),
					length: msg.len(),
					capacity: msg.capacity(),
				};
				if unsafe { !(callbackData.userCallback)(ffi, callbackData.userDataPayload.clone()) } {
					println!("FFI Call {} returned failure!", op);
				};
				Ok(())
			}
			Some(None) => Err(format!("#45 | CMD \"{}\" not registered in callback list", op)),
			None => Err(format!("#44 | CMD \"{}\" not registered in callback list", op)),
		}
	}

	/// Register a uaer defined callback
	pub fn register_callback(&mut self, swModule: &str, op: T, cb: callbackDatax) -> Result<(), String> {
		match self.db.insert(op.clone(), Some(cb)) {
			Some(_old) => {
				println!("Replacing older callback function");
				Ok(())
			}
			None => {
				println!("{} | Callback function for {} successfully registered", swModule, op);
				Ok(())
			}
		}
		//Ok(())
	}

	/// Remove a callback function
	pub fn unregister_callback(&mut self, op: T) {
		match self.db.remove(&op) {
			Some(_old) => {
				println!("Replacing older callback function");
			}
			None => {
				println!("Callback function for {} successfully registered", op);
			}
		}
	}

	// Registered opcodes
	pub fn list_registered_callbacks(&self) -> Vec<T> {
		let mut registered_callbacks: Vec<T> = Vec::new();
		println!("Registered opcodes:");
		for item in self.db.keys() {
			println!("{}", item);
			registered_callbacks.push(item.clone());
		}
		registered_callbacks
	}
}

/* impl<T> callbackDB<T>
where
	T: Display,
{
	fn default() -> Self {
		Self::new()
	}
}
 */
impl Display for callbackDatax {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Callback fn: {:?} | User Data: {:?}",
			self.userCallback, self.userDataPayload
		)
	}
}
