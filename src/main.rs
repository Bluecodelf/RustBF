extern crate rustbf;

use std::env;
use std::process;

use rustbf::config::Config;

fn main() {
    let cfg = Config::new(&mut env::args()).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1);
    });

    if let Err(e) = rustbf::run(cfg) {
        println!("{}", e);
        process::exit(1);
    }
}