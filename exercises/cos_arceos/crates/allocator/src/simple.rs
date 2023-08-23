//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    start:  usize,
    end:    usize,
    next:   usize,
    allocations:    usize
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start:  0,
            end:    0,
            next:   0,
            allocations:    0
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.start = _start;
        self.next = _start;
        self.end = _start + _size;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        self.end += _size;
        return Ok(())
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        if self.end - self.next < _layout.size() {
            return Err(crate::AllocError::NoMemory)
        }
        self.allocations += 1;
        let old_next = self.next;
        self.next += _layout.size();
        return Ok(NonZeroUsize::new(old_next).unwrap());
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.next = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        return self.end - self.start;
    }

    fn used_bytes(&self) -> usize {
        return self.next - self.start;
    }

    fn available_bytes(&self) -> usize {
        return self.end - self.next;
    }
}
