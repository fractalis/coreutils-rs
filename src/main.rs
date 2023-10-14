use std::path::PathBuf;
use std::process;

use inf_common::InfArgs;

include!(concat!(env!("OUT_DIR"), "/modules.rs"));

fn main() {
    let utils = module_map();

    let mut args = std::env::args();

    let bin_name = match args.next() {
        Some(ref s) if !s.is_empty() => PathBuf::from(s),
        _ => std::env::current_exe().unwrap(),
    };

    let bin_as_util = bin_name.file_stem().unwrap().to_str().unwrap_or_else(|| {
        process::exit(0);
    });

    let sub_command = match args.next() {
        Some(s) => s,
        _ => {
            println!("Usage: {} <subcommand>", bin_as_util);
            process::exit(0);
        }
    };

    println!("subcommand: {}", sub_command);

    let util_name = if let Some(util) = utils.keys().find(|&k| *k == sub_command.as_ref() as &str) {
        util
    } else {
        println!("Unknown subcommand: {}", sub_command);
        process::exit(0);
    };

    match utils.get(util_name) {
        Some(&(infmain, _)) => {
            process::exit(infmain(InfArgs {}));
        }
        None => {
            if *util_name == "--help" {
                println!("Help");
            }
        }
    }
}
