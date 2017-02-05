#[derive(Debug)]
pub enum Command {
    PtrIncrement,
    PtrDecrement,
    DataIncrement,
    DataDecrement,
    Output,
    Input,
    Loop(Ast),
    End,
    Unknown,
}

impl Command {
    pub fn from_chars<T>(chars: &mut T) -> Option<Command>
        where T: Iterator<Item = char>
    {
        if let Some(c) = chars.next() {
            match c {
                '>' => Some(Command::PtrIncrement),
                '<' => Some(Command::PtrDecrement),
                '+' => Some(Command::DataIncrement),
                '-' => Some(Command::DataDecrement),
                '.' => Some(Command::Output),
                ',' => Some(Command::Input),
                '[' => Some(Command::Loop(Ast::from_chars(chars))),
                ']' => Some(Command::End),
                _ => None,
            }
        } else {
            Some(Command::End)
        }
    }
}

#[derive(Debug)]
pub struct Ast(Vec<Command>);

impl Ast {
    pub fn new() -> Ast {
        Ast(Vec::new())
    }

    pub fn from_string(s: &str) -> Ast {
        Ast::from_chars(&mut s.chars())
    }

    fn from_chars<T>(chars: &mut T) -> Ast where T: Iterator<Item = char> {
        let mut ast = Ast::new();

        loop {
            if let Some(cmd) = Command::from_chars(chars) {
                match cmd {
                    Command::End => break,
                    _ => ast.append(cmd),
                };
            }
        }
        ast
    }

    pub fn append(&mut self, cmd: Command) {
        self.0.push(cmd);
    }
}