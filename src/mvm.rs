mod op_code;
use op_code::Op;

mod memory;
use memory::*;

fn eval(code: &[u8]) -> u8 {
    let mut memory = Memory::new(code);

    // registers
    let mut pc: usize = 1;

    loop {

        let op:u8 = memory.get(pc);
        pc += 1;

        match Op::from_primitive(op) {
            Op::NOP => {},// nop
            Op::XIT => {
                let a = memory.get1(pc);
                return memory.get::<u8>(a);
            },

            //
            // Integer arithmetic
            //

            Op::ADD1 => {
                let [a, b] = memory.get2(pc);
                let aa: u8 = memory.get(a);
                let bb: u8 = memory.get(b);
                memory.set(a, aa + bb);
            },
            Op::ADD2 => {
                let [a, b] = memory.get2(pc);
                let aa: u16 = memory.get(a);
                let bb: u16 = memory.get(b);
                memory.set(a, aa + bb);
            },
            Op::ADD3 => {
                let [a, b] = memory.get2(pc);
                let aa: u32 = memory.get(a);
                let bb: u32 = memory.get(b);
                memory.set(a, aa + bb);
            },
            Op::ADD4 => {
                let [a, b] = memory.get2(pc);
                let aa: u64 = memory.get(a);
                let bb: u64 = memory.get(b);
                memory.set(a, aa + bb);
            },

            Op::SUB1 => {
                let [a, b] = memory.get2(pc);
                let aa: u8 = memory.get(a);
                let bb: u8 = memory.get(b);
                memory.set(a, aa - bb);
            },
            Op::SUB2 => {
                let [a, b] = memory.get2(pc);
                let aa: u16 = memory.get(a);
                let bb: u16 = memory.get(b);
                memory.set(a, aa - bb);
            },
            Op::SUB3 => {
                let [a, b] = memory.get2(pc);
                let aa: u32 = memory.get(a);
                let bb: u32 = memory.get(b);
                memory.set(a, aa - bb);
            },
            Op::SUB4 => {
                let [a, b] = memory.get2(pc);
                let aa: u64 = memory.get(a);
                let bb: u64 = memory.get(b);
                memory.set(a, aa - bb);
            },

            Op::MUL1 => {
                let [a, b] = memory.get2(pc);
                let aa: u8 = memory.get(a);
                let bb: u8 = memory.get(b);
                memory.set(a, aa * bb);
            },
            Op::MUL2 => {
                let [a, b] = memory.get2(pc);
                let aa: u16 = memory.get(a);
                let bb: u16 = memory.get(b);
                memory.set(a, aa * bb);
            },
            Op::MUL3 => {
                let [a, b] = memory.get2(pc);
                let aa: u32 = memory.get(a);
                let bb: u32 = memory.get(b);
                memory.set(a, aa * bb);
            },
            Op::MUL4 => {
                let [a, b] = memory.get2(pc);
                let aa: u64 = memory.get(a);
                let bb: u64 = memory.get(b);
                memory.set(a, aa * bb);
            },

            Op::DIV1 => {
                let [a, b] = memory.get2(pc);
                let aa: u8 = memory.get(a);
                let bb: u8 = memory.get(b);
                memory.set(a, aa / bb);
            },
            Op::DIV2 => {
                let [a, b] = memory.get2(pc);
                let aa: u16 = memory.get(a);
                let bb: u16 = memory.get(b);
                memory.set(a, aa / bb);
            },
            Op::DIV3 => {
                let [a, b] = memory.get2(pc);
                let aa: u32 = memory.get(a);
                let bb: u32 = memory.get(b);
                memory.set(a, aa / bb);
            },
            Op::DIV4 => {
                let [a, b] = memory.get2(pc);
                let aa: u64 = memory.get(a);
                let bb: u64 = memory.get(b);
                memory.set(a, aa / bb);
            },

            Op::SHR1 => {
                let [a, b] = memory.get2(pc);
                let aa: u8 = memory.get(a);
                let bb: u8 = memory.get(b);
                memory.set(a, aa >> bb);
            },
            Op::SHR2 => {
                let [a, b] = memory.get2(pc);
                let aa: u16 = memory.get(a);
                let bb: u16 = memory.get(b);
                memory.set(a, aa >> bb);
            },
            Op::SHR3 => {
                let [a, b] = memory.get2(pc);
                let aa: u32 = memory.get(a);
                let bb: u32 = memory.get(b);
                memory.set(a, aa >> bb);
            },
            Op::SHR4 => {
                let [a, b] = memory.get2(pc);
                let aa: u64 = memory.get(a);
                let bb: u64 = memory.get(b);
                memory.set(a, aa >> bb);
            },


            Op::SHL1 => {
                let [a, b] = memory.get2(pc);
                let aa: u8 = memory.get(a);
                let bb: u8 = memory.get(b);
                memory.set(a, aa << bb);
            },
            Op::SHL2 => {
                let [a, b] = memory.get2(pc);
                let aa: u16 = memory.get(a);
                let bb: u16 = memory.get(b);
                memory.set(a, aa << bb);
            },
            Op::SHL3 => {
                let [a, b] = memory.get2(pc);
                let aa: u32 = memory.get(a);
                let bb: u32 = memory.get(b);
                memory.set(a, aa << bb);
            },
            Op::SHL4 => {
                let [a, b] = memory.get2(pc);
                let aa: u64 = memory.get(a);
                let bb: u64 = memory.get(b);
                memory.set(a, aa << bb);
            },

            Op::AND1 => {
                let [a, b] = memory.get2(pc);
                let aa: u8 = memory.get(a);
                let bb: u8 = memory.get(b);
                memory.set(a, aa & bb);
            },
            Op::AND2 => {
                let [a, b] = memory.get2(pc);
                let aa: u16 = memory.get(a);
                let bb: u16 = memory.get(b);
                memory.set(a, aa & bb);
            },
            Op::AND3 => {
                let [a, b] = memory.get2(pc);
                let aa: u32 = memory.get(a);
                let bb: u32 = memory.get(b);
                memory.set(a, aa & bb);
            },
            Op::AND4 => {
                let [a, b] = memory.get2(pc);
                let aa: u64 = memory.get(a);
                let bb: u64 = memory.get(b);
                memory.set(a, aa & bb);
            },

            Op::ORR1 => {
                let [a, b] = memory.get2(pc);
                let aa: u8 = memory.get(a);
                let bb: u8 = memory.get(b);
                memory.set(a, aa | bb);
            },
            Op::ORR2 => {
                let [a, b] = memory.get2(pc);
                let aa: u16 = memory.get(a);
                let bb: u16 = memory.get(b);
                memory.set(a, aa | bb);
            },
            Op::ORR3 => {
                let [a, b] = memory.get2(pc);
                let aa: u32 = memory.get(a);
                let bb: u32 = memory.get(b);
                memory.set(a, aa | bb);
            },
            Op::ORR4 => {
                let [a, b] = memory.get2(pc);
                let aa: u64 = memory.get(a);
                let bb: u64 = memory.get(b);
                memory.set(a, aa | bb);
            },

            Op::XOR1 => {
                let [a, b] = memory.get2(pc);
                let aa: u8 = memory.get(a);
                let bb: u8 = memory.get(b);
                memory.set(a, aa ^ bb);
            },
            Op::XOR2 => {
                let [a, b] = memory.get2(pc);
                let aa: u16 = memory.get(a);
                let bb: u16 = memory.get(b);
                memory.set(a, aa ^ bb);
            },
            Op::XOR3 => {
                let [a, b] = memory.get2(pc);
                let aa: u32 = memory.get(a);
                let bb: u32 = memory.get(b);
                memory.set(a, aa ^ bb);
            },
            Op::XOR4 => {
                let [a, b] = memory.get2(pc);
                let aa: u64 = memory.get(a);
                let bb: u64 = memory.get(b);
                memory.set(a, aa ^ bb);
            },

            //
            // floating point ops
            //

            Op::ADDF1 => {
                let [a, b] = memory.get2(pc);
                let aa: f32 = memory.get(a);
                let bb: f32 = memory.get(b);
                memory.set(a, aa + bb);
            },
            Op::ADDF2 => {
                let [a, b] = memory.get2(pc);
                let aa: f64 = memory.get(a);
                let bb: f64 = memory.get(b);
                memory.set(a, aa + bb);
            },

            Op::SUBF1 => {
                let [a, b] = memory.get2(pc);
                let aa: f32 = memory.get(a);
                let bb: f32 = memory.get(b);
                memory.set(a, aa - bb);
            },
            Op::SUBF2 => {
                let [a, b] = memory.get2(pc);
                let aa: f64 = memory.get(a);
                let bb: f64 = memory.get(b);
                memory.set(a, aa - bb);
            },

            Op::MULF1 => {
                let [a, b] = memory.get2(pc);
                let aa: f32 = memory.get(a);
                let bb: f32 = memory.get(b);
                memory.set(a, aa * bb);
            },
            Op::MULF2 => {
                let [a, b] = memory.get2(pc);
                let aa: f64 = memory.get(a);
                let bb: f64 = memory.get(b);
                memory.set(a, aa * bb);
            },

            Op::DIVF1 => {
                let [a, b] = memory.get2(pc);
                let aa: f32 = memory.get(a);
                let bb: f32 = memory.get(b);
                memory.set(a, aa / bb);
            },
            Op::DIVF2 => {
                let [a, b] = memory.get2(pc);
                let aa: f64 = memory.get(a);
                let bb: f64 = memory.get(b);
                memory.set(a, aa / bb);
            },

            Op::CPY1 => {
                let [a, b] = memory.get2(pc);
                let bb: u8 = memory.get(b);
                memory.set(a, bb);
            },
            Op::CPY2 => {
                let [a, b] = memory.get2(pc);
                let bb: u16 = memory.get(b);
                memory.set(a, bb);
            },
            Op::CPY3 => {
                let [a, b] = memory.get2(pc);
                let bb: u32 = memory.get(b);
                memory.set(a, bb);
            },
            Op::CPY4 => {
                let [a, b] = memory.get2(pc);
                let bb: u64 = memory.get(b);
                memory.set(a, bb);
            },

            Op::CPA1 => {
                let [a, b] = memory.get2(pc);
                let bb: usize = memory.get(b);
                let bbb: u8 = memory.get(bb);
                memory.set(a, bbb);
            },
            Op::CPA2 => {
                let [a, b] = memory.get2(pc);
                let bb: usize = memory.get(b);
                let bbb: u16 = memory.get(bb);
                memory.set(a, bbb);
            },
            Op::CPA3 => {
                let [a, b] = memory.get2(pc);
                let bb: usize = memory.get(b);
                let bbb: u32 = memory.get(bb);
                memory.set(a, bbb);
            },
            Op::CPA4 => {
                let [a, b] = memory.get2(pc);
                let bb: usize = memory.get(b);
                let bbb: u64 = memory.get(bb);
                memory.set(a, bbb);
            },

            //
            // control flow
            //

            Op::JMP => {
                pc = memory.get1(pc);
            },
            Op::JIT => {
                let [loc, val] = memory.get2(pc);
                if memory.get::<u64>(val) != 0 {
                    pc = memory.get1(loc);
                };
            },
            Op::CAL => {
                let loc = memory.get1(pc);
                memory.push_addr(pc);
                pc = loc;
            },
            Op::RET => {
                pc = memory.pop_addr();
            }

            //
            // page management
            //

            Op::ALP => {
                let loc = memory.alloc_page();
                memory.set(pc, loc);
            },
            Op::FRP => {
                let loc = memory.get1(pc);
                memory.free_page(loc);
            },

            Op::ASY => {
                panic!("async not implemented")
            }

            Op::EXT => {
                panic!("extensions not implemented");
            }
        }
    }
}

fn main() {
    eval(&[1]);
    println!("Hello, world!");
}
