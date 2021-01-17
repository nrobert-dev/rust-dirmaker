
pub mod conf;

use std::env;
use std::process;

use crate::conf::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing {}", err);
        process::exit(1);
    });

    if let Err(e) = config.run(){
        println!("Application error : {}",e);
        process::exit(1);
    }
}
