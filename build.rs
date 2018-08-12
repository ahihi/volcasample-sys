extern crate bindgen;
extern crate cc;

use std::{error::Error, env, path::PathBuf};

fn build() -> Result<(), Box<Error>> {
    cc::Build::new()
        .file("vendor/pattern/volcasample_pattern.c")
        .file("vendor/syro/korg_syro_comp.c")
        .file("vendor/syro/korg_syro_func.c")
        .file("vendor/syro/korg_syro_volcasample.c")
        .compile("volcasample");

    let bindings = bindgen::Builder::default()
        .header("vendor/syro/korg_syro_type.h")
        .header("vendor/syro/korg_syro_volcasample.h")
        .header("vendor/syro/korg_syro_comp.h")
        .header("vendor/syro/korg_syro_func.h")
        .header("vendor/pattern/volcasample_pattern.h")
        .whitelist_type("Endian")
        .whitelist_type("Syro.+")
        .whitelist_type("VolcaSample_.+")
        .whitelist_function("SyroComp_.+")
        .whitelist_function("SyroFunc_.+")
        .whitelist_function("SyroVolcaSample_.+")
        .whitelist_var("VOLCASAMPLE_.+")
        .rustfmt_bindings(true)
        .generate()
        .map_err(|()| "Failed to generate bindings")?;

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings
        .write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}

fn main() {
    match build() {
        Ok(()) => {},
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}
