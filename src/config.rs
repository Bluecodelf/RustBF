#[derive(Debug)]
pub enum Operation {
    PrintAst,
    TranslateC,
    Execute,
}

pub struct Config {
    pub path: String,
    pub operation: Operation,
}

impl Config {
    pub fn new<T>(args: &mut T) -> Result<Config, &'static str>
        where T: Iterator<Item = String>
    {
        args.next();

        let mut path: Option<String> = None;
        let mut operation = Operation::PrintAst;
        for arg in args {
            match &arg[..] {
                "--toc" => operation = Operation::TranslateC,
                "--ast" => operation = Operation::PrintAst,
                "--exec" => operation = Operation::Execute,
                _ => path = Some(arg),
            };
        }

        match path {
            Some(f) => Ok(Config {
                path: f,
                operation: operation,
            }),
            None => Err("Please specify a file.")
        }
    }
}