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
        .flag_if_supported("-fwrapv");

    if windows {
        cfg.define("_WIN32", None);
    }

    cfg.compile("cubiomes");
}
