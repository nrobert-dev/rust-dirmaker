use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs;


pub struct Config{
    path : String,
    root : String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
             Err("not enough arguments supplied to function!")
        } else {
            let path = args[1].clone();
            let root = args[2].clone();
            Ok(Config { path, root })
        }
    }


    pub fn run(&self) -> Result<(), Box<dyn Error>>{
        let file = fs::File::open(&self.path)?;
        let reader = BufReader::new(file);
    
        for(_index, line) in reader.lines().enumerate(){
            let new_path = Path::new(&self.root).join(line?);
            println!("Created folder : {}", new_path.display());
    
            fs::create_dir_all(new_path)?;
        }
        Ok(())
    }
    
}