const KB:usize = 1024;
const MB:usize = KB * KB;

const FAST_SIZE: usize = 64 * KB;
const PAGE_SIZE: usize = 4 * KB;
const CACHE_LINE: usize = 64;

/*
 * basic op codes:
 *  arguments are pointers
 *
 *  opc a1  a2  a3
 *  --------------
 *  nop
 *                  integer arith 1-4 variants
 *  add dst arg
 *  sub dst arg
 *  div dst arg
 *  sub dst arg
 *                  float arithm 1-2 variants
 *  addf dst arg
 *  subf dst arg
 *  mulf dst arg
 *  divf dst arg
 *
 *  cpy dst src     1-4 variants, *dst = *src
 *  cpa dst ptr     1-4 variants, *dst = **src
 *
 *  jmp loc
 *  jiz loc if0
 *  alc dst         alloc page
 *  dlc src         dealloc page
 *
 */

#[allow(dead_code)]
enum Op {
    // misc
    NOP, XIT,

    // integers
    ADD1, ADD2, ADD3, ADD4,
    SUB1, SUB2, SUB3, SUB4,
    MUL1, MUL2, MUL3, MUL4,
    DIV1, DIV2, DIV3, DIV4,
    // bitwise ints
    SHR1, SHR2, SHR3, SHR4,
    SHL1, SHL2, SHL3, SHL4,
    // bitwise ops
    AND1, AND2, AND3, AND4,
    ORR1, ORR2, ORR3, ORR4,
    XOR1, XOR2, XOR3, XOR4,

    // floats
    ADDF1, ADDF2,
    SUBF1, SUBF2,
    MULF1, MULF2,
    DIVF1, DIVF2,

    // copy pointed to
    CPY1, CPY2, CPY3, CPY4,

    // copy pointed to to
    CPA1, CPA2, CPA3, CPA4,

    // turing completenes
    JMP, JIT, CAL, RET,

    // alloc/free page
    ALP, FRP,

    // async
    ASY,

    // extension codes
    EXT,
}

impl Op {
    fn from_primitive(n: u8) -> Op {
        return unsafe { std::mem::transmute::<u8, Op>(n) };
    }
}

struct Memory {
    fast: [u8; FAST_SIZE],
    page: Vec<Option<[u8; PAGE_SIZE]>>,
    sp: usize,

}

impl Memory {
    fn new(code: &[u8]) -> Memory {
        let mut ret = Memory{
            fast: [0; FAST_SIZE],
            page: vec![],
            sp: FAST_SIZE + CACHE_LINE - 1,
        };

        ret.fast[1..code.len()+1].clone_from_slice(code);

        return ret;
    }

    fn alloc_page(&mut self) -> usize {

        for (i, item) in self.page.iter().enumerate() {
            if item.is_none() {
                self.page[i] = Some([0; PAGE_SIZE]);
                    return CACHE_LINE + FAST_SIZE + (i * PAGE_SIZE);
            }
        }

        let i = self.page.len();
        self.page.push(Some([0; PAGE_SIZE]));

        return CACHE_LINE + FAST_SIZE + (i * PAGE_SIZE);
    }

    fn free_page(&mut self, addr:usize) {
        let i = (addr - CACHE_LINE - FAST_SIZE) / PAGE_SIZE;
        
        std::mem::drop(self.page[i].expect("page does not exist"));

        self.page[i] = None;

    }

    fn push_addr(&mut self, addr: usize) {
        self.set(self.sp, addr);
        self.sp -= 8;
    }

    fn pop_addr(&mut self) -> usize {
        self.sp -= 8;
        return self.get1(self.sp);
    }


    #[inline]
    fn get<T: Clone>(&self, addr: usize) -> T {

        /*
         * addressing scheme:
         * 0                    __ null
         * |
         * |
         * CACHE_LINE-1         __ relative addresses
         * |
         * |
         * FAST_SIZE+CACHE_LINE __ fast memory
         * |
         * |
         * :                    __ page memory
         */
        let addr1: usize; // addres space without relative chunk
        let ptr: *const u8; // pointer to location we are looking for

        if addr == 0 {
            panic!("null pointer deref");
        } else if addr <= CACHE_LINE {
            addr1 = self.sp - addr;
        } else {
            addr1 = addr - CACHE_LINE;
        }

        // set the pointer
        if addr1 < FAST_SIZE {
            ptr = &self.fast[addr];
        } else {
            let addr1 = addr1 - FAST_SIZE;
            let page_num = addr1 >> 12; // page size bit
            let page_idx = addr1 & (PAGE_SIZE - 1);

            ptr = &self.page[page_num]
                .expect("page not exist")[page_idx];
        };

        return unsafe { (*(ptr as *const T)).clone() };
    }

    #[inline]
    fn get1(&self, addr: usize) -> usize {
        return self.get::<usize>(addr);
    }

    #[inline]
    fn get2(&self, addr: usize) -> [usize; 2] {
        return self.get::<[usize; 2]>(addr);
    }

    // #[inline]
    // fn get3(&self, addr: usize) -> [usize; 3] {
    //     return self.get::<[usize; 3]>(addr);
    // }


    #[inline]
    fn set<T: Clone>(&mut self, addr: usize, val: T) {
        let addr1: usize; // addres space without relative chunk
        let ptr: *mut u8; // pointer to location we are looking for

        if addr == 0 {
            panic!("null pointer deref");
        } else if addr <= CACHE_LINE {
            addr1 = self.sp - addr;
        } else {
            addr1 = addr - CACHE_LINE;
        }

        // set the pointer
        if addr1 < FAST_SIZE {
            ptr = &mut self.fast[addr];
        } else {
            let addr1 = addr1 - FAST_SIZE;
            let page_num = addr1 >> 12; // page size bit
            let page_idx = addr1 & (PAGE_SIZE - 1);

            ptr = &mut self.page[page_num]
                .expect("page not exist")[page_idx];
        };

        unsafe {
            *(ptr as *mut T) = val.clone()
        };
    }
}

fn eval(code: &[u8]) -> u8 {
    let mut memory = Memory::new(code);

    println!("{:?}", memory.get::<u32>(0));
    memory.set(0, 0u32);
    println!("{:?}", memory.get::<u32>(0));
    memory.set(0, 0xffffu32);
    println!("{:?}", memory.get::<u32>(0));
    memory.set(0, 0x0u8);
    println!("{:?}", memory.get::<u32>(0));

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
                memory.free_page(memory.get1(pc));
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
