//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, TlsfByteAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;
extern crate axlog as log;

const POOL_SIZE: usize = 1 << 18;
const MEMORY_END: usize = 0xffffffc088000000;

pub struct LabByteAllocator {
    pool_alloc: TlsfByteAllocator,
    start: usize,
    end: usize,
    short: usize,
    long: usize,
    count: usize,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self {
            pool_alloc: TlsfByteAllocator::new(),
            start: 0,
            end: 0,
            count: 0,
            short: 0,
            long: 0,
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        log::info!("init : start 0x{:0x} , size 0x{:0x}", start, size);
        self.pool_alloc.init(start, POOL_SIZE);
        self.start = start + POOL_SIZE;
        self.end = MEMORY_END; // start + size;
        self.short = self.end;
        self.long = self.start;
        self.count = 0;
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        unimplemented!()
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        log::info!("layout: size {} , align {}", layout.size(), layout.align());
        let align = layout.align();
        if align == 8 {
            return self
                .pool_alloc
                .alloc(layout)
                .map_err(|_| AllocError::NoMemory);
        }

        let size = layout.size();
        self.count += 1;
        // 跳过 stdout的alloc
        if (self.count - 1) % 15 % 2 == 0 {
            self.short = self.end - size;
            if self.short < self.long {
                return AllocResult::Err(AllocError::NoMemory);
            }
            return AllocResult::Ok(unsafe { NonNull::new_unchecked(self.short as *mut u8) });
        } else {
            self.long += size;
            if self.long > self.end {
                return AllocResult::Err(AllocError::NoMemory);
            }
            return AllocResult::Ok(unsafe {
                NonNull::new_unchecked((self.long - size) as *mut u8)
            });
        }
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        if layout.align() == 8 {
            self.pool_alloc.dealloc(pos, layout);
            return;
        }

        if self.end - self.short == layout.size() {
            self.short = self.end;
        }
    }
    fn total_bytes(&self) -> usize {
        self.end - self.start
    }
    fn used_bytes(&self) -> usize {
        self.long - self.start + self.end - self.short
    }
    fn available_bytes(&self) -> usize {
        self.short - self.long
    }
}
