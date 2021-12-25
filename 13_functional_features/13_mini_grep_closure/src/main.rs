use std::{env, process };
use mini_grep_closure::Config;


fn main() {
    let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    if let Err(e) =  mini_grep_closure::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


