use ast::{Ast, Command};

pub fn to_c(ast: &Ast) {
    let mut indent = 0u32;

    print_c_line("#include <stdio.h>", indent);
    print_c_line("int main()\n{", indent);
    indent += 1;
    print_c_line("char array[30000] = {0};", indent);
    print_c_line("char *ptr = array;", indent);
    to_c_rec(&mut ast.iter(), indent);
    print_c_line("return 0;", indent);
    indent -= 1;
    print_c_line("}", indent);
}

fn to_c_rec<'a, T>(iter: &'a mut T, indent: u32)
    where T: Iterator<Item = &'a Command>
{
    for cmd in iter {
        match cmd {
            &Command::PtrIncrement => print_c_line("++ptr;", indent),
            &Command::PtrDecrement => print_c_line("--ptr;", indent),
            &Command::DataIncrement => print_c_line("++*ptr;", indent),
            &Command::DataDecrement => print_c_line("--*ptr;", indent),
            &Command::Output => print_c_line("putchar(*ptr);", indent),
            &Command::Input => print_c_line("*ptr=getchar();", indent),
            &Command::Loop(ref sub_ast) => {
                print_c_line("while (*ptr)", indent);
                print_c_line("{", indent);
                to_c_rec(&mut sub_ast.iter(), indent + 1);
                print_c_line("}", indent);
            },
            _ => (),
        };
    }
}

fn print_c_line(line: &str, indent: u32) {
    for _ in 0..indent {
        print!("    ");
    }
    println!("{}", line);
}