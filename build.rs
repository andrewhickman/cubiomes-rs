use std::{env, path::PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();
    let windows = target.contains("windows");

    let mut cfg = cc::Build::new();
    cfg.include("cubiomes")
        .file("cubiomes/biome_tree.c")
        .file("cubiomes/finders.c")
        .file("cubiomes/generator.c")
        .file("cubiomes/layers.c")
        .file("cubiomes/noise.c")
        .file("cubiomes/util.c")
        .flag("-fwrapv");

    if windows {
        cfg.define("_WIN32", None);
    }

    cfg.compile("cubiomes");

    let bindings = bindgen::Builder::default()
        .header("cubiomes/generator.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
