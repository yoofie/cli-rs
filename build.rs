use std::env;
use std::path::PathBuf;

fn main() -> shadow_rs::SdResult<()> {
	env::set_var("RUST_BACKTRACE", "1");
	env::set_var("CBINDGEN_OUTPUT", "pubApi/");
	let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var is not defined");
	let out_dir = PathBuf::from(env::var("CBINDGEN_OUTPUT").expect("CBINDGEN_OUTPUT env var is not defined"));

	let c_config = cbindgen::Config::from_file("./support/cbindgen.toml")
		.expect("Unable to find cbindgen.toml configuration file");
	let cpp_config = cbindgen::Config::from_file("./support/cppbindgen.toml")
		.expect("Unable to find cppbindgen.toml configuration file");

	cbindgen::generate_with_config(&crate_dir, c_config)
		.unwrap()
		.write_to_file(out_dir.join("pubApi_c.h"));
	cbindgen::generate_with_config(&crate_dir, cpp_config)
		.unwrap()
		.write_to_file(out_dir.join("pubApi_cpp.h"));
	shadow_rs::new()
}
