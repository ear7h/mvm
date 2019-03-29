/* .label
 *      directive "mem static" ; directive gets
 *                             ; treated as a command all the way
 *                             ; to the parser
 * ._SECTION_LABEL_
 * .label1 ; comment
 *      COMMAND 1 2
 *      command one two ; comments
 * .label2
 *      command
 */
/*
 * after parsing the file, the assembler compiles a binary file
 * by:
 *      -read nodes one by one put labels in a map
 */

use std::iter::Peekable;
use std::str::Chars;
use std::collections::HashMap;

mod op_code;
use op_code::Op;

macro_rules! dense_enum {
    ($name:ident;
        $($var:ident) , * ,
    ) => {
        #[allow(dead_code)]
        #[derive(Debug)]
        pub enum $name {
            $($var) , *
        }

        impl $name {
            pub fn from_int(i:u8) -> $name {
                return unsafe { std::mem::transmute::<u8, $name>(i) }
            }

            pub fn to_str(&self) -> &'static str {
                match self {
                    $($name::$var => stringify!($var)) , *
                }
            }

            pub fn from_string(s: &String) -> Result<$name, String> {
                match s.to_uppercase().as_str() {
                    $(stringify!($var) => Ok($name::$var)) , * ,
                    _ => Err(format!("{} not a[n] {}", s, stringify!($name))),
                }
            }
        }
    }
}

#[derive(Debug)]
enum Value {
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
enum AstNode {
    Tree(String, Vec<AstNode>), // name of program and the nodes
    Cmd(AsmCmd, Vec<Value>), // command and arguments
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
            let cmd_res = AsmCmd::from_string(&cmd);
            let cmd_cmd: AsmCmd;
            match cmd_res {
                Err(x) => return Err(x),
                Ok(x) => cmd_cmd = x,
            }
            consumeln_ws(chars);

            // read args
            let mut args = Vec::new(); // return arr
            let mut arg = String::new(); // current arg
            'outer: while let Some(&c) = chars.peek() {
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
                    'a'...'z' | 'A'...'Z' | '0'...'9' => {
                        arg.push(c);
                        chars.next();
                    },
                    '"' => {
                        arg.push(c);
                        chars.next();
                        while let Some(c) = chars.next() {
                                arg.push(c);
                            if c == '"' {
                                continue 'outer;
                            }
                        }

                        return Err("unclosed \"".to_string())
                    },
                    '\n' | ';' => break, // command line ends at \n or comment
                    _ => return Err(
                        format!("unexpected char parsing command {}", c)),
                }
            }

            if !arg.is_empty() {
                let argv = Value::parse(arg.clone());
                if let Value::Err(x) = argv {
                    return Err(x);
                }

                args.push(argv);
            }

            Ok(AstNode::Cmd(cmd_cmd, args))
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

fn parse(src: String) -> Result<AstNode, String> {
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

fn build(tree:&AstNode) -> Vec<u8> {
    let mut ops:Vec<Op> = Vec::new();
    let mut labels: HashMap<String, &usize> = HashMap::new();

    return vec![];
}

/**
 * Assembler commands. These relate to op codes in the VM, but the op
 * codes are picked implicitly based on the arguments.
 */
dense_enum! { AsmCmd;
    NOP, XIT,

    // byte
    ADDB, SUBB, MULB, DIVB,
    SHRB, SHLB,
    ANDB, ORRB, XORB,

    // word
    ADDW, SUBW, MULW, DIVW,
    SHRW, SHLW,
    ANDW, ORRW, XORW,

    ADDF, SUBF, MULF, DIVF,

    CPYB, CPAB,
    CPYW, CPAW,
    JMP, JIT, CAL, RET,

    PSHB, POPB,
    PSHW, POPW,

    // extension codes
    APG, FPG,
    ASY,
    CMT,

    STTC,
}

impl AsmCmd {

    fn compile<'a>(&self,
               args: &Vec<Value>,
               labels: &'a mut HashMap<String, usize>)
        -> Result<Vec<u8>, String> {

        match self {
            NOP => {
                Ok(vec![Op::NOP as u8])
            },
            XIT => {
                Ok(vec![Op::XIT as u8])
            },
            ADDB => {
                if args.len() != 2 {
                    return Err(
                        format!("expected 2 args to  ADDB got {:?}", args));
                };

                let dst = match &args[0] {
                    Value::Label(x) => {
                        if let Some(&y) = labels.get(x) {
                            y
                        } else {
                            return Err(format!("label {} not defined", x))
                        }
                    },
                    Value::Addr(x) => *x,
                    _ => return Err(
                        format!("dst arg must be addr-like got {:?}", args[0])),
                };

                match &args[1] {
                    Value::Addr(src) => {
                        let mut ret = vec![Op::ADD3 as u8];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        ret.extend_from_slice(&src.to_le_bytes());
                        Ok(ret)
                    },
                    Value::Label(x) => {
                        let mut ret = vec![Op::ADD3 as u8];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        if let Some(src) = labels.get(x) {
                            ret.extend_from_slice(&src.to_le_bytes());
                            Ok(ret)
                        } else {
                            Err(format!("label {} not defined", x))
                        }
                    },
                    Value::Int(src) => {
                        let mut ret = vec![Op::ADD1 as u8];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        ret.push(*src as u8);
                        Ok(ret)
                    },
                    Value::Uint(src) => {
                        let mut ret = vec![Op::ADD1 as u8];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        ret.push(*src as u8);
                        Ok(ret)
                    },
                    _ => Err(
                        format!("unexpected arg {:?}", args[1])),
                }
            },
            ADDW => {
                if args.len() != 2 {
                    return Err(
                        format!("expected 2 args to  ADDB got {:?}", args));
                };

                let dst = match &args[0] {
                    Value::Label(x) => {
                        if let Some(&y) = labels.get(x) {
                            y
                        } else {
                            return Err(format!("label {} not defined", x))
                        }
                    },
                    Value::Addr(x) => *x,
                    _ => return Err(
                        format!("dst arg must be addr-like got {:?}", args[0])),
                };

                match &args[1] {
                    Value::Addr(src) => {
                        let mut ret = vec![Op::ADD4 as u8];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        ret.extend_from_slice(&src.to_le_bytes());
                        Ok(ret)
                    },
                    Value::Label(x) => {
                        let mut ret = vec![Op::ADD4 as u8];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        if let Some(src) = labels.get(x) {
                            ret.extend_from_slice(&src.to_le_bytes());
                            Ok(ret)
                        } else {
                            Err(format!("label {} not defined", x))
                        }
                    },
                    Value::Int(src) => {
                        let mut ret = vec![Op::ADD2 as u8];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        ret.extend_from_slice(&src.to_le_bytes());
                        Ok(ret)
                    },
                    Value::Uint(src) => {
                        let mut ret = vec![Op::ADD2 as u8];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        ret.extend_from_slice(&src.to_le_bytes());
                        Ok(ret)
                    },
                    _ => Err(
                        format!("unexpected arg {:?}", args[1])),
                }
            },
            _ => panic!(format!("unknown command {:?}", self)),
        }
    }
}




const prog:&str = "; our nifty little program
.start
    FAD arg1 arg2
\tFSU  arg2
    FAD ; comment 2
    STTC \"asd\"
    ; comment 3";

fn main() {
    println!("{}", prog);
    let res = parse(prog.to_string());
    println!("{:?}", res);
}
