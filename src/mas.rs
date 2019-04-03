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


use std::collections::HashMap;
use std::fmt;

mod op_code;
use op_code::Op;

mod memory;

mod ast;
use ast::Value;
use ast::AstNode;

macro_rules! dense_enum {
    ($name:ident;
        $($var:ident) , * ,) => {
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

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.to_str())
            }
        }
    }
}


fn compile(root:&AstNode) -> Result<Vec<u8>, String> {
    let mut ops:Vec<Op> = Vec::new();
    let mut labels: HashMap<String, usize> = HashMap::new();

    // builtin labels
    labels.insert("._zero".to_string(), memory::PROG_OFFSET);

    let nodes: &Vec<AstNode>;
    if let AstNode::Tree(_, nodes1) = root {
        nodes = nodes1;
    } else {
        panic!(format!("root is not AstNode::Tree, got {:?}", root))
    }

    let mut prog_size = 1usize; // the 0 byte is for an exit code

    // fill labels
    for node in nodes {
        match node {
            AstNode::Cmd(cmd, args) => {
                prog_size += match AsmCmd::size_from_string(cmd, args) {
                    Ok(size) => size,
                    Err(msg) => return Err(msg),
                };
            },
            AstNode::Label(name) => {
                match labels.get(name) {
                    Some(addr) => {
                        return Err(format!("label already defined {:?}", node))
                    },
                    None => {
                        labels.insert(name.to_string(),
                            prog_size + memory::PROG_OFFSET);
                    },
                };
            },
            _ => {}, // ignore comments
        }
    }

    // returned vector
    let mut ret = Vec::with_capacity(prog_size);

    // actually compile the program
    for node in nodes {
        match node {
            AstNode::Cmd(cmd, args) => {
                let instr = match AsmCmd::from_string(cmd) {
                    Ok(cmd) => cmd.compile(args, &labels),
                    Err(msg) => return Err(msg),
                };
                match instr {
                    Ok(byt) => ret.extend(byt.iter()),
                    Err(_) => return instr,
                };
            },
            _ => {}, // ignore comments
        }
    }

    return Ok(ret)
}

/**
 * Assembler commands. These relate to op codes in the VM, but the op
 * codes are picked implicitly based on the arguments.
 */
dense_enum! { AsmCmd;
    NOP, XIT,

    // byte
    ADDB, SUBB, MULB, DIVB, MODB,
    SHRB, SHLB,
    ANDB, ORRB, XORB,

    // word
    ADDW, SUBW, MULW, DIVW, MODW,
    SHRW, SHLW,
    ANDW, ORRW, XORW,

    ADDF, SUBF, MULF, DIVF,

    CPYB, CPYW,
    JMP, JIT, CAL, RET,

    PSHB, POPB,
    PSHW, POPW,

    // extension codes
//    APG, FPG,
//    ASY,
//   CMT,

    // STTC, // values in binary
    // ALLO, // values requested
}

impl AsmCmd {

    fn base_op_code(&self) -> u8 {
        return match *self {
            AsmCmd::ADDB | AsmCmd::ADDW => Op::ADD1,
            AsmCmd::SUBB | AsmCmd::SUBW => Op::SUB1,
            AsmCmd::MULB | AsmCmd::MULW => Op::MUL1,
            AsmCmd::DIVB | AsmCmd::DIVW => Op::DIV1,
            AsmCmd::MODB | AsmCmd::MODW => Op::MOD1,
            AsmCmd::SHRB | AsmCmd::SHRW => Op::SHR1,
            AsmCmd::SHLB | AsmCmd::SHLW => Op::SHL1,
            AsmCmd::ANDB | AsmCmd::ANDW => Op::AND1,
            AsmCmd::ORRB | AsmCmd::ORRW => Op::ORR1,
            AsmCmd::XORB | AsmCmd::XORW => Op::XOR1,
            AsmCmd::CPYB | AsmCmd::CPYW => Op::CPY1,
            AsmCmd::PSHB | AsmCmd::PSHW => Op::PSH1,
            AsmCmd::POPB | AsmCmd::POPW => Op::POP1,
            _ => panic!("no base op for {}", self),
        } as u8
    }

    /**
     * offset based on size of the data being operated
     */
    fn base_op_offset(&self) -> u8 {
        match self {
            // byte
            AsmCmd::ADDB | AsmCmd::SUBB | AsmCmd::MULB | AsmCmd::DIVB | AsmCmd::MODB |
            AsmCmd::SHRB | AsmCmd::SHLB |
            AsmCmd::ANDB | AsmCmd::ORRB | AsmCmd::XORB |
            AsmCmd::CPYB |
            AsmCmd::PSHB | AsmCmd::POPW => 0,

            // word
            AsmCmd::ADDW | AsmCmd::SUBW | AsmCmd::MULW | AsmCmd::DIVW | AsmCmd::MODW |
            AsmCmd::SHRW | AsmCmd::SHLW |
            AsmCmd::ANDW | AsmCmd::ORRW | AsmCmd::XORW |
            AsmCmd::CPYW |
            AsmCmd::PSHW | AsmCmd::POPW => 2,

            _ => panic!(format!("no offset for {}", self))
        }
    }

    fn size_from_string(cmd: &String, args: &Vec<Value>)
        -> Result<usize, String> {

        let cmd1 = match AsmCmd::from_string(cmd) {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        let ret = match cmd1 {
            AsmCmd::NOP | AsmCmd::XIT | AsmCmd::RET=> 1, // no arg
            AsmCmd::JMP | AsmCmd::CAL => 1 + 8, // op code and jump address
            AsmCmd::JIT => 1 + 8 + 8, // op code jump address, boolean address

            AsmCmd::ADDB | AsmCmd::SUBB |
            AsmCmd::MULB | AsmCmd::DIVB | AsmCmd::MODB |
            AsmCmd::SHRB | AsmCmd::SHLB |
            AsmCmd::ANDB | AsmCmd::ORRB | AsmCmd::XORB |
            AsmCmd::CPYB => {

                if args.len() != 2 {
                    return Err(
                        format!("expected 2 args to {} got {:?}", cmd1, args));
                };

                // op code + dst + (src | val)
                1 + 8 + match args[1] {
                    Value::Label(_) | Value::Addr(_) => 8,
                    Value::Int(_) | Value::Uint(_) => 1,
                    _ => return Err(format!("unexpected argument {:?}", args[1])),
                }
            },

            AsmCmd::ADDW | AsmCmd::SUBW |
            AsmCmd::MULW | AsmCmd::DIVW | AsmCmd::MODW |
            AsmCmd::SHRW | AsmCmd::SHLW |
            AsmCmd::ANDW | AsmCmd::ORRW | AsmCmd::XORW |
            AsmCmd::CPYW => {

                if args.len() != 2 {
                    return Err(
                        format!("expected 2 args to {} got {:?}", cmd1, args));
                };

                // op code + dst + (src | val)
                1 + 8 + match args[1] {
                    Value::Label(_) | Value::Addr(_) => 8,
                    Value::Int(_) | Value::Uint(_) => 8,
                    _ => return Err(format!("unexpected argument {:?}", args[1])),
                }
            },

            AsmCmd::ADDF | AsmCmd::SUBF | AsmCmd::MULF | AsmCmd::DIVF => {
                if args.len() != 2 {
                    return Err(
                        format!("expected 2 args to {} got {:?}", cmd1, args));
                };

                // op code + dst + (src | val)
                1 + 8 + match args[1] {
                    Value::Label(_) | Value::Addr(_) => 8,
                    Value::Int(_) | Value::Uint(_) => 4,
                    _ => return Err(format!("unexpected argument {:?}", args[1])),
                }
            },
            AsmCmd::PSHB => {
                if args.len() != 1 {
                    return Err(
                        format!("expected 1 args to {} got {:?}", cmd1, args));
                };

                1 + match args[1] {
                    Value::Label(_) | Value::Addr(_) => 8,
                    Value::Int(_) | Value::Uint(_) => 1,
                    _ => return Err(format!("unexpected argument {:?}", args[1])),
                }
            },
            AsmCmd::PSHW => {
                if args.len() != 1 {
                    return Err(
                        format!("expected 1 args to {} got {:?}", cmd1, args));
                };

                1 + match args[1] {
                    Value::Label(_) | Value::Addr(_) => 8,
                    Value::Int(_) | Value::Uint(_) => 8,
                    _ => return Err(format!("unexpected argument {:?}", args[1])),
                }
            },
            AsmCmd::POPB | AsmCmd::POPW => {
                if args.len() != 1 {
                    return Err(
                        format!("expected 1 args to {} got {:?}", cmd1, args));
                };

                1 + match args[1] {
                    Value::Label(_) | Value::Addr(_) => 8,
                    _ => return Err(format!("unexpected argument {:?}", args[1])),
                }
            },
            _ => return panic!(format!("{} does not have an op code", cmd1))
        };

        return Ok(ret)
    }

    fn compile<'a>(&self,
               args: &Vec<Value>,
               labels: &'a HashMap<String, usize>)
        -> Result<Vec<u8>, String> {

        match self {
            AsmCmd::NOP => {
                Ok(vec![Op::NOP as u8])
            },
            AsmCmd::XIT => {
                Ok(vec![Op::XIT as u8])
            },

            // byte
            AsmCmd::ADDB | AsmCmd::SUBB |
            AsmCmd::MULB | AsmCmd::DIVB | AsmCmd::MODB |
            AsmCmd::SHRB | AsmCmd::SHLB |
            AsmCmd::ANDB | AsmCmd::ORRB | AsmCmd::XORB |
            AsmCmd::CPYB |

            // word
            AsmCmd::ADDW | AsmCmd::SUBW |
            AsmCmd::MULW | AsmCmd::DIVW | AsmCmd::MODW |
            AsmCmd::SHRW | AsmCmd::SHLW |
            AsmCmd::ANDW | AsmCmd::ORRW | AsmCmd::XORW |
            AsmCmd::CPYW => {
                if args.len() != 2 {
                    return Err(
                        format!("expected 2 args to {} got {:?}", self, args));
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
                        let mut ret =
                            vec![self.base_op_code() + self.base_op_offset() + 1];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        ret.extend_from_slice(&src.to_le_bytes());
                        Ok(ret)
                    },
                    Value::Label(x) => {
                        let mut ret =
                            vec![self.base_op_code() + self.base_op_offset() + 1];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        if let Some(src) = labels.get(x) {
                            ret.extend_from_slice(&src.to_le_bytes());
                            Ok(ret)
                        } else {
                            Err(format!("label {} not defined", x))
                        }
                    },
                    Value::Int(src) => {
                        let mut ret =
                            vec![self.base_op_code() + self.base_op_offset()];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        ret.extend_from_slice(
                            &src
                            .to_le_bytes()[
                                ..if 0 == self.base_op_offset() {1} else {8}]);
                        Ok(ret)
                    },
                    Value::Uint(src) => {
                        let mut ret =
                            vec![self.base_op_code() + self.base_op_offset()];
                        ret.extend_from_slice(&dst.to_le_bytes());
                        ret.extend_from_slice(
                            &src
                            .to_le_bytes()[
                                ..if 0 == self.base_op_offset() {1} else {8}]);
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



const prog2:&str = "; program 2
.start
    addb ._zero 10 ; asd
    modb ._zero 7  ; another line
    xit
.globals";

fn main() {

    println!("{}", prog2);
    let res2 = ast::parse(prog2.to_string());
    println!("{:?}", res2);

    println!("{:?}", compile(&res2.unwrap()));

}
