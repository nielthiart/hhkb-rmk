use std::io::Read;
use std::path::PathBuf;
use std::{env, fs};

use const_gen::{CompileConst, const_declaration};
use xz2::read::XzEncoder;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Vial reads the keyboard definition from the firmware, xz-compressed.
    println!("cargo:rerun-if-changed=vial.json");
    let vial_json = fs::read_to_string("vial.json").expect("vial.json should be readable");
    let vial_json =
        json::stringify(json::parse(&vial_json).expect("vial.json should be valid JSON"));
    let mut vial_def = Vec::new();
    XzEncoder::new(vial_json.as_bytes(), 6)
        .read_to_end(&mut vial_def)
        .unwrap();
    let vial_id: Vec<u8> = vec![0xC6, 0x5A, 0x71, 0x7F, 0x80, 0x48, 0xAD, 0xD9];
    let consts = [
        const_declaration!(pub VIAL_KEYBOARD_DEF = vial_def),
        const_declaration!(pub VIAL_KEYBOARD_ID = vial_id),
    ];
    fs::write(out.join("config_generated.rs"), consts.join("\n")).unwrap();

    fs::write(out.join("memory.x"), include_bytes!("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    // The FLASH origin is not 0x10000 aligned.
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
}
