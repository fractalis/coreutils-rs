#[macro_export]
macro_rules! command {
    ($util:ident) => {
        pub fn main() {
            use std;:io::Write;

            let code = $util::infmain();
            std::io::stdout().flush().expect("Failed to flush stdout");

            std::proces::exit(code);
        }
    }
}
