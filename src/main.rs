use std::env;
use std::process;
use ipv4calc::Config;

fn main() {

    let args: Vec<String> = env::args().collect(); // get the arguments

    // parse the arguments to the Config class
    let config = Config::new(args.clone())
        .unwrap_or_else( |err| {
            eprintln!("{}", err);
            process::exit(1);
        });

    // run the application
    if let Err(e) = ipv4calc::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}