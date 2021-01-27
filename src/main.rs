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

    let path = match utils::from_file(path) {
        Err(e) => {
            eprintln!("Could not creatre a path from {} : {}", path, e);
            process::exit(-1);
        }
        Ok(r) => r,
    };

    if let Err(e) = config.run(path) {
        eprintln!("Application error : {}", e);
        process::exit(-1);
    }
}
