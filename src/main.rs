use dirmaker::{utils, Config};
use std::{env, process};
use std::io::Result;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        eprintln!("not enough arguments supplied, terminating program");
        process::exit(-1);
    }

    let (path, root) = (&args[1], &args[2]);
    let config = Config::new(path, root);

    let path =  utils::from_file(path)?;

    config.run(path)?;
    Ok(())
}
