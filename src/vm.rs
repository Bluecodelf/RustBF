use std::io;
use std::error::Error;
use std::io::prelude::*;

use ast::{Ast, Command};

pub struct VirtualMachine {
    data: [u8; 30000],
    ptr: usize,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            data: [0; 30000],
            ptr: 0,
        }
    }

    pub fn execute(&mut self, ast: &Ast) -> Result<(), Box<Error>> {
        self.execute_rec(&mut ast.iter())
    }

    fn execute_rec<'a, T>(&mut self, iter: &'a mut T) -> Result<(), Box<Error>>
        where T: Iterator<Item = &'a Command>
    {
        for cmd in iter {
            match cmd {
                &Command::PtrIncrement => self.ptr += 1,
                &Command::PtrDecrement => self.ptr -= 1,
                &Command::DataIncrement => self.data[self.ptr] += 1,
                &Command::DataDecrement => self.data[self.ptr] -= 1,
                &Command::Output => print!("{}", self.data[self.ptr] as char),
                &Command::Input => {
                    let mut buf: [u8; 1] = [0; 1];
                    io::stdin().read_exact(&mut buf)?;
                    self.data[self.ptr] = buf[0];
                }
                &Command::Loop(ref sub_ast) => {
                    while self.data[self.ptr] != 0u8 {
                        if let Err(result) = self.execute_rec(&mut sub_ast.iter()) {
                            return Err(result);
                        }
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
}