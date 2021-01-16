use std::env;
use std::fs;
use std::process;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;


struct Config{
    path : String,
    root : String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments supplied to function!")
        }
        let path = args[1].clone();
        let root = args[2].clone();
        Ok(Config { path, root })
    }
}

fn run(config : Config) -> Result<(), Box<dyn Error>>{
    let file = fs::File::open(config.path)?;
    let reader = BufReader::new(file);

    for(_index, line) in reader.lines().enumerate(){
        let new_path = Path::new(&config.root).join(line?);
        println!("Created folder : {}", new_path.display());

        fs::create_dir_all(new_path)?;
    }
    Ok(())
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let config : Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config){
        println!("Application error : {}",e);
        process::exit(1);
    }
}
