use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum Value {
    Label(String),
    Addr(usize),
    Int(i64),
    Uint(u64),
    Float(f32),
    Str(String),
    Err(String), // reults in a lex error
}

impl Value {
    fn parse(s: String) -> Value {
        let first = s.as_bytes()[0] as char;
        match first {
            '0'...'9' => {
                if let Ok(num) = s.parse::<i64>() {
                    Value::Int(num)
                } else if let Ok(num) = s.parse::<u64>() {
                    Value::Uint(num)
                } else if let Ok(num) = s.parse::<f32>() {
                    Value::Float(num)
                } else {
                    Value::Err(format!("unexpected string {}", s))
                }
            },
            '"' => {
                // remove the quotes
                Value::Str(s.get(1..s.len()-1).unwrap().to_string())
            },
            '.' => return Value::Label(s),
            '&' => {
                match s.get(1..).unwrap().to_string().parse::<usize>() {
                    Ok(x) => Value::Addr(x),
                    Err(x) => Value::Err(format!("bad address {}", s))
                }
            },
            // maybe someday this will be used for registers
            _ => Value::Err(format!("unexpected string {}", s)),
        }
    }
}

#[derive(Debug)]
pub enum AstNode {
    Tree(String, Vec<AstNode>), // name of program and the nodes
    Cmd(String, Vec<Value>), // command and arguments
    Label(String), // name
    Comment(String),
}


impl AstNode {
    fn parse_cmd(chars: &mut Peekable<Chars>)
        -> Result<AstNode, String> {

            // read cmd
            let mut cmd = String::new();
            while let Some(&c) = chars.peek() {
                match c {
                    ' ' | '\t' | '\n' => break,
                    _ => cmd.push(chars.next().unwrap()),
                }
            }

            consumeln_ws(chars);

            // read args
            let mut args = Vec::new(); // return arr
            let mut arg = String::new(); // current arg
            while let Some(&c) = chars.peek() {
                match c {
                    ' ' | '\t' => {
                        let argv = Value::parse(arg.clone());
                        if let Value::Err(x) = argv {
                            return Err(x);
                        }

                        args.push(argv);
                        arg.clear();
                        consumeln_ws(chars);
                    },
                    '\n' | ';' => break, // command line ends at \n or comment
                    _ => {
                        arg.push(c);
                        chars.next();
                    },
                }
            }

            if !arg.is_empty() {
                let argv = Value::parse(arg.clone());
                if let Value::Err(x) = argv {
                    return Err(x);
                }

                args.push(argv);
            }

            Ok(AstNode::Cmd(cmd, args))
    }

    fn parse_label(chars: &mut Peekable<Chars>)
        -> Result<AstNode, String>  {

            assert_eq!(chars.peek(), Some(&'.'));

            let mut label = String::new();

            while let Some(c) = chars.next() {
                match c {
                    ' ' | '\t' => {
                        consume_ws(chars);
                        break
                    },
                    '\n' => break,
                    ':' => continue, // allow : for readability
                    _ => label.push(c),
                }
            }

            Ok(AstNode::Label(label))
    }

    fn parse_comment(chars: &mut Peekable<Chars>)
        -> Result<AstNode, String>  {

            assert_eq!(chars.next(), Some(';'));

            let mut body = String::new();

            while let Some(c) = chars.next() {
                match c {
                    '\n' => break,
                    _ => body.push(c),
                }
            }

            Ok(AstNode::Comment(body))
    }
}


fn consumeln_ws(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        if c == ' ' || c == '\t' {
            chars.next();
        } else {
            return;
        }
    }
}

fn consume_ws(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        if c == ' ' || c == '\t' || c == '\n' {
            chars.next();
        } else {
            return;
        }
    }
}

pub fn parse(src: String) -> Result<AstNode, String> {
    let name = "anon".to_string();
    let mut nodes = Vec::new();

    let mut chars = src.chars().peekable();
    while let Some(&c) = chars.peek() {
        let mut res: Result<AstNode, String>;
        match c {
            'a'...'z' | 'A'...'Z' => {
                res = AstNode::parse_cmd(&mut chars);
            },
            '.' => {
                res = AstNode::parse_label(&mut chars);
            },
            ';' => {
                res = AstNode::parse_comment(&mut chars);
            },
            '\n' => {
                chars.next();
                continue
            },
            _ => return Err(format!("unexpected char {}", c)),
        }
        consume_ws(&mut chars);

        match res {
            Ok(x) => nodes.push(x),
            Err(x) => return Err(x),
        }
    }

    Ok(AstNode::Tree(name, nodes))
}
