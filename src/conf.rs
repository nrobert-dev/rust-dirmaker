use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs;


pub struct Config{
    path : String,
    root : String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        if args.len() < 3 {
             panic!("not enough arguments supplied, terminating program")
        } 
            let path = args[1].clone();
            let root = args[2].clone();
            Config { path, root }
    }


    pub fn run(&self) -> Result<(), Box<dyn Error>>{
        let file = fs::File::open(&self.path)?;
        let reader = BufReader::new(file);
    
        for(_index, line) in reader.lines().enumerate(){
            let new_path = Path::new(&self.root).join(line?);    
            fs::create_dir_all(&new_path)?;
            println!("Created folder : {}", new_path.display());

        }
        Ok(())
    }
    
}