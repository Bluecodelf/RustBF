use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub mod ast;
pub mod config;
pub mod c;

use config::{Config, Operation};

pub fn run(cfg: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(cfg.path)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let ast = ast::Ast::from_string(&contents);

    match cfg.operation {
        Operation::PrintAst => println!("{:#?}", ast),
        Operation::TranslateC => c::to_c(&ast),
    }

    Ok(())
}