// This file is part of the my coreutils-rs package.
//
// It is used to generate a stub to load the modules from the
// coreutils-rs crate.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={profile:?}");
    }

    const ENV_FEATURE_PREFIX: &str = "CARGO_FEATURE_";
    const FEATURE_PREFIX: &str = "feat_";
    const COMMAND_PREFIX: &str = "inf_";

    let out_dir = env::var("OUT_DIR").unwrap();
    dbg!(&out_dir);

    let mut commands = Vec::new();

    for (key, val) in env::vars() {
        if val == "1" && key.starts_with(ENV_FEATURE_PREFIX) {
            let feature = key[ENV_FEATURE_PREFIX.len()..].to_lowercase();

            match feature.as_ref() {
                "default" | "unix" => continue,
                "test" => continue,
                s if s.starts_with(FEATURE_PREFIX) => continue,
                _ => {}
            }
            commands.push(feature);
        }
    }
    commands.sort();

    dbg!(&commands);

    let mut mf = File::create(Path::new(&out_dir).join("modules.rs")).unwrap();

    mf.write_all(
        "type ModuleMap = phf::OrderedMap<&'static str, (fn(inf_common::InfArgs) -> i32, fn() -> i32)>;\n\n\
            #[allow(clippy::too_many_lines)]
            fn module_map() -> ModuleMap {\n"
            .as_bytes(),
    )
    .unwrap();

    let mut phf_map = phf_codegen::OrderedMap::<&str>::new();
    for command in &commands {
        let command_value = format!("({command}::infmain, {command}::inf_app)");
        match command.as_ref() as &str {
            k if k.starts_with(COMMAND_PREFIX) => {
                phf_map.entry(&k[COMMAND_PREFIX.len()..], &command_value);
            }
            _ => {
                phf_map.entry(command, &command_value);
            }
        }
    }
    write!(mf, "{}", phf_map.build()).unwrap();
    mf.write_all(b"\n}\n").unwrap();

    mf.flush().unwrap();
}
