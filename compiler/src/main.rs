use std::{env, eprintln, process};
use compilex::{CompileConfig, compile};
fn main() {
    let config = CompileConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let _s = match compile(config) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
}
