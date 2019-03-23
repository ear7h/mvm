const KB:usize = 1024;
const MB:usize = KB * KB;

const FAST_SIZE: usize = 4 * MB;
const PAGE_SIZE: usize = 4 * KB;

struct Memory {
    fast: [u8; FAST_SIZE],
    page: Vec<[u8; PAGE_SIZE]>,
}

impl Memory {
    fn new(code: &[u8]) -> Memory {
        let mut ret = Memory{
            fast: [0; FAST_SIZE],
        page: vec![],
        };

        ret.fast[..code.len()].clone_from_slice(code);

        return ret;
    }

    fn alloc_page(&mut self) {
        self.page.push([0; 4 * KB]);
    }

    fn free_page(&mut self) {
        self.page.pop();
    }


    #[inline]
    fn get<T: Clone>(&mut self, addr: usize) -> T {
        let ptr: *mut u8;

        if addr < FAST_SIZE {
            ptr = &mut self.fast[addr];
        } else {
            let addr1 = addr - FAST_SIZE;
            let page_num = addr1 >> 12; // page size bit
            let page_idx = addr1 & (PAGE_SIZE - 1);

            ptr = &mut self.page[page_num][page_idx];
        };

        return unsafe { (*(ptr as *mut T)).clone() };
    }


    #[inline]
    fn set<T: Clone>(&mut self, addr: usize, val: T) {
        let ptr: *mut u8;

        if addr < FAST_SIZE {
            ptr = &mut self.fast[addr];
        } else {
            let addr1 = addr - FAST_SIZE;
            let page_num = addr1 >> 12; // page size bit
            let page_idx = addr1 & (PAGE_SIZE - 1);

            ptr = &mut self.page[page_num][page_idx];
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


    return 0;
}

fn main() {
    eval(&[1]);
    println!("Hello, world!");
}
