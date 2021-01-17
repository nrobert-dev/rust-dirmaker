
pub mod conf;

use std::env;
use std::process;

use crate::conf::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args);

    if let Err(e) = config.run(){
        println!("Application error : {}",e);
        process::exit(1);
    }
}
