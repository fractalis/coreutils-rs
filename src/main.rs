include!(concat!(env!("OUT_DIR"), "/modules.rs"));

fn main() {
    let utils = module_map();

    let mut args = std::env::args();

    dbg!(args);
}
