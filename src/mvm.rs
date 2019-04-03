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
        println!("{:?}", Op::from_int(op));
        pc += 1;

        match Op::from_int(op) {
            Op::NOP => {},// nop
            Op::XIT => {
                return memory.get::<u8>(memory::PROG_OFFSET);
            },

            //
            // Integer arithmetic
            //

            Op::ADD1 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(pc);
                pc += 1;
                memory.set(dst, dstv + srcv);
            },
            Op::ADD2 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv + srcv);
            },
            Op::ADD3 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u64 = memory.get(dst);
                let srcv: u64 = memory.get(pc);
                pc += 8;
                memory.set(dst, dstv + srcv);
            },
            Op::ADD4 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv + srcv);
            },

            Op::SUB1 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(pc);
                pc += 1;
                memory.set(dst, dstv - srcv);
            },
            Op::SUB2 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv - srcv);
            },
            Op::SUB3 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u64 = memory.get(dst);
                let srcv: u64 = memory.get(pc);
                pc += 8;
                memory.set(dst, dstv - srcv);
            },
            Op::SUB4 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv - srcv);
            },

            Op::MUL1 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(pc);
                pc += 1;
                memory.set(dst, dstv * srcv);
            },
            Op::MUL2 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv * srcv);
            },
            Op::MUL3 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u64 = memory.get(dst);
                let srcv: u64 = memory.get(pc);
                pc += 8;
                memory.set(dst, dstv * srcv);
            },
            Op::MUL4 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv * srcv);
            },

            Op::DIV1 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(pc);
                pc += 1;
                memory.set(dst, dstv / srcv);
            },
            Op::DIV2 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv / srcv);
            },
            Op::DIV3 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u64 = memory.get(dst);
                let srcv: u64 = memory.get(pc);
                pc += 8;
                memory.set(dst, dstv / srcv);
            },
            Op::DIV4 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv / srcv);
            },

            Op::MOD1 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(pc);
                pc += 1;
                memory.set(dst, dstv % srcv);
            },
            Op::MOD2 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv % srcv);
            },
            Op::MOD3 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u64 = memory.get(dst);
                let srcv: u64 = memory.get(pc);
                pc += 8;
                memory.set(dst, dstv % srcv);
            },
            Op::MOD4 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv % srcv);
            },

            Op::SHR1 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(pc);
                pc += 1;
                memory.set(dst, dstv >> srcv);
            },
            Op::SHR2 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv >> srcv);
            },
            Op::SHR3 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u64 = memory.get(dst);
                let srcv: u64 = memory.get(pc);
                pc += 8;
                memory.set(dst, dstv >> srcv);
            },
            Op::SHR4 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv >> srcv);
            },

            Op::SHL1 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(pc);
                pc += 1;
                memory.set(dst, dstv << srcv);
            },
            Op::SHL2 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv << srcv);
            },
            Op::SHL3 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u64 = memory.get(dst);
                let srcv: u64 = memory.get(pc);
                pc += 8;
                memory.set(dst, dstv << srcv);
            },
            Op::SHL4 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv << srcv);
            },


            Op::AND1 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(pc);
                pc += 1;
                memory.set(dst, dstv & srcv);
            },
            Op::AND2 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv & srcv);
            },
            Op::AND3 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u64 = memory.get(dst);
                let srcv: u64 = memory.get(pc);
                pc += 8;
                memory.set(dst, dstv & srcv);
            },
            Op::AND4 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv & srcv);
            },

            Op::ORR1 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(pc);
                pc += 1;
                memory.set(dst, dstv | srcv);
            },
            Op::ORR2 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv | srcv);
            },
            Op::ORR3 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u64 = memory.get(dst);
                let srcv: u64 = memory.get(pc);
                pc += 8;
                memory.set(dst, dstv | srcv);
            },
            Op::ORR4 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv | srcv);
            },

            Op::XOR1 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(pc);
                pc += 1;
                memory.set(dst, dstv ^ srcv);
            },
            Op::XOR2 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv ^ srcv);
            },
            Op::XOR3 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let dstv: u64 = memory.get(dst);
                let srcv: u64 = memory.get(pc);
                pc += 8;
                memory.set(dst, dstv ^ srcv);
            },
            Op::XOR4 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let dstv: u8 = memory.get(dst);
                let srcv: u8 = memory.get(src);
                memory.set(dst, dstv ^ srcv);
            },

            //
            // floating point ops
            //

            Op::ADDF => {
                let [a, b] = memory.get2(pc);
                let aa: f32 = memory.get(a);
                let bb: f32 = memory.get(b);
                memory.set(a, aa + bb);
            },
            Op::SUBF => {
                let [a, b] = memory.get2(pc);
                let aa: f32 = memory.get(a);
                let bb: f32 = memory.get(b);
                memory.set(a, aa - bb);
            },
            Op::MULF => {
                let [a, b] = memory.get2(pc);
                let aa: f32 = memory.get(a);
                let bb: f32 = memory.get(b);
                memory.set(a, aa * bb);
            },
            Op::DIVF => {
                let [a, b] = memory.get2(pc);
                let aa: f32 = memory.get(a);
                let bb: f32 = memory.get(b);
                memory.set(a, aa / bb);
            },

            //
            // memory operations
            //

            Op::CPY1 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let srcv: u8 = memory.get(pc);
                pc += 1;
                memory.set(dst, srcv);
            },
            Op::CPY2 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let srcv: u8 = memory.get(src);
                memory.set(dst, srcv);
            },
            Op::CPY3 => {
                let dst: usize = memory.get(pc);
                pc += 8;
                let srcv: u64 = memory.get(pc);
                pc += 8;
                memory.set(dst, srcv);
            },
            Op::CPY4 => {
                let [dst, src] = memory.get2(pc);
                pc += 16;
                let srcv: u8 = memory.get(src);
                memory.set(dst, srcv);
            },

            code => panic!(format!("{:?} not implemented", code)),

        }
    }

    println!("{:?}", memory.get::<[u8; 32]>(0));
}

fn main() {
    let s = eval(&[
                    Op::ADD1 as u8, 64, 0, 0, 0, 0, 0, 0, 0, 10,
                    Op::MOD1 as u8, 64, 0, 0, 0, 0, 0, 0, 0, 7,
                    Op::XIT as u8]);
    println!("{}", s);
    std::process::exit(s as i32);
}
