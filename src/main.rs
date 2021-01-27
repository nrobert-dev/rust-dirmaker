
use dirmaker::{utils, Config};
use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        eprintln!("not enough arguments supplied, terminating program");
        process::exit(-1);
    }

    let (path, root) = (&args[1], &args[2]);
    let config = Config::new(path, root);

    if let Err(e) = config.run(utils::from_file(path).unwrap()) {
        eprintln!("Application error : {}", e);
        process::exit(-1);
    }
}
