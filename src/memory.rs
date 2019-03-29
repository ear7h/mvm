pub const KB:usize = 1024;
pub const MB:usize = KB * KB;

pub const FAST_SIZE: usize = 32 * KB;
pub const PAGE_SIZE: usize = 4 * KB;
pub const CACHE_LINE: usize = 64;

pub struct Memory {
    fast: [u8; FAST_SIZE],
    page: Vec<Option<[u8; PAGE_SIZE]>>,
    sp: usize,

}

impl Memory {
    pub fn new(code: &[u8]) -> Memory {
        let mut ret = Memory{
            fast: [0; FAST_SIZE],
            page: vec![],
            sp: FAST_SIZE + CACHE_LINE - 1,
        };

        ret.fast[1..code.len()+1].clone_from_slice(code);

        return ret;
    }

    pub fn alloc_page(&mut self) -> usize {

        for (i, item) in self.page.iter_mut().enumerate() {
            if item.is_none() {
                item.replace([0; PAGE_SIZE]);

                return CACHE_LINE + FAST_SIZE + (i * PAGE_SIZE);
            }
        }

        let i = self.page.len();
        self.page.push(Some([0; PAGE_SIZE]));

        return CACHE_LINE + FAST_SIZE + (i * PAGE_SIZE);
    }

    pub fn free_page(&mut self, addr:usize) {
        let i = (addr - CACHE_LINE - FAST_SIZE) / PAGE_SIZE;

        std::mem::drop(self.page[i].expect("page does not exist"));

        self.page[i] = None;

    }

    pub fn push_addr(&mut self, addr: usize) {
        let loc = self.sp;
        self.set(loc, addr);
        self.sp -= 8;
    }

    pub fn pop_addr(&mut self) -> usize {
        self.sp -= 8;
        return self.get1(self.sp);
    }

    pub fn get<T: Clone>(&self, addr: usize) -> T {

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

    pub fn get1(&self, addr: usize) -> usize {
        return self.get::<usize>(addr);
    }

    pub fn get2(&self, addr: usize) -> [usize; 2] {
        return self.get::<[usize; 2]>(addr);
    }

    pub fn set<T: Clone>(&mut self, addr: usize, val: T) {
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
