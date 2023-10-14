#[inf_common::main]
pub fn infmain(args: inf_common::InfArgs) -> i32 {
    println!("Hello, world - infmain");

    let mut app = inf_app();

    println!("Hello, world - infmain - app = {}", app);
    0
}

pub fn inf_app() -> i32 {
    println!("Hello, world - inf_app");
    0
}
