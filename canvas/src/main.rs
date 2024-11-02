use canvas::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    if let Err(e) = canvas::run(config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}