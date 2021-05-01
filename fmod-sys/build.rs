extern crate bindgen;

use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("cty")
        .clang_arg(
            "-Ic:/Program Files (x86)/FMOD SoundSystem/FMOD Studio API Windows/api/studio/inc",
        )
        .clang_arg(
            "-Ic:/Program Files (x86)/FMOD SoundSystem/FMOD Studio API Windows/api/core/inc",
        )
        .header("wrapper.h")
        .constified_enum_module("FMOD_STUDIO_LOADING_STATE")
        .constified_enum_module("FMOD_STUDIO_LOAD_MEMORY_MODE")
        .constified_enum_module("FMOD_STUDIO_PARAMETER_TYPE")
        .constified_enum_module("FMOD_STUDIO_USER_PROPERTY_TYPE")
        .constified_enum_module("FMOD_STUDIO_EVENT_PROPERTY")
        .constified_enum_module("FMOD_STUDIO_PLAYBACK_STATE")
        .constified_enum_module("FMOD_STUDIO_STOP_MODE")
        .constified_enum_module("FMOD_STUDIO_INSTANCETYPE")
        .constified_enum_module("FMOD_RESULT")
        .generate()
        .expect("unable to generate binding")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
}
