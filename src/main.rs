mod config;
mod functions;
mod integrate;

use std::env;
use std::process::exit;
use config::Config;
use integrate::{parallel_integrate_err};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} path/to/config", args[0]);
        exit(1);
    }
    let mut config = Config::new(&args[1]);
    let res = parallel_integrate_err(&mut config);
    println!("{:?}", res);
}
