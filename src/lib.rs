use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub mod ast;

pub struct Config {
    path: String,
}

impl Config {
    pub fn new<T>(args: &mut T) -> Result<Config, &'static str>
        where T: Iterator<Item = String>
    {
        args.next();
        match args.next() {
            Some(arg) => Ok(Config { path: arg }),
            None => Err("You must specify a file."),
        }
    }
}

pub fn run(cfg: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(cfg.path)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let ast = ast::Ast::from_string(&contents);
    println!("{:#?}", ast);

    Ok(())
}