

use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs;


pub struct Config{
    path : String,
    root : String,
}

impl Config {
    pub fn new(root : &str, path : &str) -> Self {
            Config { path : path.to_owned(), root: root.to_owned() }
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



#[cfg(test)]
mod tests {
    use crate::conf::Config;
    #[test]
    fn test_root_and_path() {
        let cf = Config::new("./","tomi.txt");
        println!("{} {}", cf.path, cf.root);
        assert_eq!(cf.root,"./");
        assert_eq!(cf.path,"tomi.txt");
    }
}