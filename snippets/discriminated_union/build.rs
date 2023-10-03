extern crate cbindgen;

use std::env;

use cbindgen::Config;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    //panic!("{}", std::env::var("HOST").unwrap());

    let mut config = Config::default();
    config.language = cbindgen::Language::C;
    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("generated_by_buildrs.h");
}
