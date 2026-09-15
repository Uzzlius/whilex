use std::{env, eprintln, process};
use while_lang::{Config, run};
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let _s = match run(config) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
}
