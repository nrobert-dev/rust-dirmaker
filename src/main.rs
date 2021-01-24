
pub mod conf;

use std::env;
use crate::conf::{Config, utils};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        panic!("not enough arguments supplied, terminating program");
    }

    let (path, root) = (&args[1], &args[2]);
     
    let config = Config::new(path, root);

    if let Err(e) = config.run(utils::from_file(path).unwrap()){
        panic!("Application error : {}",e);
    }
}
