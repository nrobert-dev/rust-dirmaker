

use std::error::Error;
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


    pub fn run(&self, entries :  Box<dyn Iterator<Item=String>>) -> Result<(), Box<dyn Error>>{
        for line in entries{
            let new_path = Path::new(&self.root).join(line);    
            fs::create_dir_all(&new_path)?;
            println!("Created folder : {}", new_path.display());
        }
        Ok(())
    }
    
}



pub mod utils {
    use std::io::{BufRead, BufReader};
    use std::fs::File;
    use std::error::Error;

    pub fn from_file(file_path : &str) ->  Result<Box<dyn Iterator<Item=String>>, Box<dyn Error>> {
        let lines: Result<Vec<_>, _> = { 
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);

            reader.lines().collect() 
        };
        let lines = lines?;
        
        Ok(Box::new(lines.into_iter()))
    }

}



#[cfg(test)]
mod tests {
    use crate::conf::Config;
    #[test]
    fn test_root_and_path() {
        //arrange 
        let cf = Config::new("./","tomi.txt");

        //act 

        //assert
        assert_eq!(cf.root,"./");
        assert_eq!(cf.path,"tomi.txt");
    }
}