use std::fs;
use std::io::Result;
use std::path::Path;

pub struct Config {
    path: &'static Path,
    root: String,
}

impl Config {
    pub fn new(root: &str, path: &'static str) -> Self {
        Config {
            path: Path::new(path),
            root: root.to_owned(),
        }
    }

    pub fn run(&self, entries: impl IntoIterator<Item = String>) -> Result<()> {
        for line in entries {
            let new_path = Path::new(&self.root).join(line);
            fs::create_dir_all(&new_path)?;
            println!("Created folder : {}", new_path.display());
        }
        Ok(())
    }
}

pub mod utils {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Result};

    pub fn from_file(file_path: &str) -> Result<impl IntoIterator<Item = String>> {
        let lines = {
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);

            reader.lines().collect::<Result<Vec<_>>>()?
        };
        Ok(lines.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use crate::Config;
    #[test]
    fn test_root_and_path() {
        //arrange
        let cf = Config::new("./", "tomi.txt");

        //act

        //assert
        assert_eq!(cf.root, "./");
        assert_eq!(cf.path, "tomi.txt");
    }
}
