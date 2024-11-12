use core::alloc::GlobalAlloc;

struct Heap<const S:usize>{
    memory: [u8;S]
}

unsafe impl<const S:usize> GlobalAlloc for Heap<S>{
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}

#[global_allocator]
static HEAP: Heap<1024> = Heap{memory: [0;1024]};
