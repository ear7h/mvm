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

            pub fn from_string(s: String) -> Result<$name, String> {
                match s.to_uppercase().as_str() {
                    $(stringify!($var) => Ok($name::$var)) , * ,
                    _ => Err(format!("{} not a[n] {}", s, stringify!($name))),
                }
            }
        }
    }
}

/*
 * 1 -> 1 byte       left is ptr, right is val
 * 2 -> 1 byte       left + right are ptrs
 * 3 -> 8 bytes      left is ptr, right is val
 * 4 -> 8 bytes      left + right are ptrs
 */
dense_enum! { Op;
    // misc
    NOP, XIT,

    // integers
    ADD1, ADD2, ADD3, ADD4,
    SUB1, SUB2, SUB3, SUB4,
    MUL1, MUL2, MUL3, MUL4,
    DIV1, DIV2, DIV3, DIV4,
    // bitwise shift
    SHR1, SHR2, SHR3, SHR4,
    SHL1, SHL2, SHL3, SHL4,
    // bitwise ops
    AND1, AND2, AND3, AND4,
    ORR1, ORR2, ORR3, ORR4,
    XOR1, XOR2, XOR3, XOR4,

    // floats
    ADDF, SUBF, MULF, DIVF,

    // copy pointed to
    CPY1, CPY2, CPY3, CPY4,
    // copy pointed to to
    CPA1, CPA2, CPA3, CPA4,

    // turing completenes
    JMP1, JMP2, // 1: static address, 2: dynamic address
    JIT, CAL, RET,

    // stack
    PSH1, PSH2, PSH3, PSH4,
    POP1, POP2, POP3, POP4,

    // extension codes
    EXT,
}

#[allow(dead_code)]
pub enum OpExt {
    // sleep
    SLP,

    // alloc/free page
    APG, FPG,

    // async
    ASY,

    // comment, possibly for debug info
    CMT
}

