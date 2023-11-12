use std::env;
use std::path::PathBuf;

fn main() {
	env::set_var("RUST_BACKTRACE", "1");
	env::set_var("CBINDGEN_OUTPUT", "bin/");
	let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var is not defined");
	let out_dir = PathBuf::from(env::var("CBINDGEN_OUTPUT").expect("CBINDGEN_OUTPUT env var is not defined"));

	let config = cbindgen::Config::from_file("cbindgen.toml").expect("Unable to find cbindgen.toml configuration file");

	cbindgen::generate_with_config(&crate_dir, config)
		.unwrap()
		.write_to_file(out_dir.join("pubApi.h"));
}
