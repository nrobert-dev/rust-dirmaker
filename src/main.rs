use dirmaker::{utils, Config};
use std::{env, process};
use std::io::Result;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();


    let config = match args.len() {
        0 | 1 => {
            eprintln!("Not enough arguments supplied, terminating program");
            process::exit(-1);
        }
        2 => {
            Config::new_default(&args[1])
        }
        3 => {
            let (path, root) = (&args[1], &args[2]);
            Config::new(root, path)
        }
        _ => {
            //TODO decision, how to handle more than 3 arguments passed to the command
            eprintln!("Wrong number of arguments supplied");
            process::exit(-1); 
        }
    };

    let paths = utils::paths_from_file(&args[1])?;

    match config {
        Ok(config) => config.run(paths)?,
        Err(error) => eprintln!("{:?}", error)
    }

    Ok(())
}
