extern crate bindgen;
extern crate cc;

use std::{error::Error, env, path::PathBuf};

fn build() -> Result<(), Box<Error>> {
    cc::Build::new()
        .file("vendor/syro/korg_syro_comp.c")
        .file("vendor/syro/korg_syro_func.c")
        .file("vendor/syro/korg_syro_volcasample.c")
        .compile("syro_volcasample");

    let bindings = bindgen::Builder::default()
        .header("vendor/syro/korg_syro_type.h")
        .header("vendor/syro/korg_syro_volcasample.h")
        .header("vendor/syro/korg_syro_comp.h")
        .header("vendor/syro/korg_syro_func.h")
        .whitelist_type("Endian")
        .whitelist_type("Syro.+")
        .whitelist_function("SyroComp_.+")
        .whitelist_function("SyroFunc_.+")
        .whitelist_function("SyroVolcaSample_.+")
        .whitelist_var("VOLCASAMPLE_COMP_BLOCK_LEN")
        .whitelist_var("VOLCASAMPLE_NUM_OF_PATTERN")
        .whitelist_var("VOLCASAMPLE_NUM_OF_SAMPLE")
        .whitelist_var("VOLCASAMPLE_PATTERN_SIZE")

        
        //.whitelist_type("Endian")
        //.whitelist_type("SyroDataType")
        //.whitelist_function()
        //.whitelist_var("Endian")
        //.whitelist_var
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
