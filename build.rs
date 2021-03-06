extern crate bindgen;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-Ideps/libossia/src/ossia-c/")
        .constified_enum_module("ossia_access_mode")
        .constified_enum_module("ossia_bounding_mode")
        .constified_enum_module("ossia_type")
        .constified_enum_module("log_level")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let dst = Config::new("deps/libossia/")
        .define("OSSIA_C", "ON")
        .define("OSSIA_DATAFLOW", "OFF")
        .define("OSSIA_EDITOR", "OFF")
        .define("OSSIA_PROTOCOL_ARTNET", "OFF")
        .define("OSSIA_PROTOCOL_AUDIO", "OFF")
        .define("OSSIA_PROTOCOL_MIDI", "OFF")
        .define("OSSIA_STATIC", "ON")
        .define("RTMIDI17_NO_WINUWP", "OFF")
        .build_target("all")
        .build();
    println!(
        "cargo:rustc-link-search=native={}/build/src/",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=ossia");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
