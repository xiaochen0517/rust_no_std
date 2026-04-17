#![allow(warnings)]

static HEAP: [u8; 1024 * 1024] = [0; 1024 * 1024];
static mut HEAP_INIT: bool = false;
static mut HEAP_START: usize = 0;
static mut HEAP_PTR: usize = 0;

#[repr(C)]
struct BlockHeader {
    size: usize,   // 8 bytes
    is_free: bool, // 1 byte, 7 bytes padding
} // Total: 16 bytes

pub unsafe fn init_heap() {
    if HEAP_INIT {
        return;
    }
    HEAP_START = HEAP.as_ptr() as usize;
    HEAP_PTR = HEAP_START;
    HEAP_INIT = true;
}

///
/// 从堆中分配内存
/// size: 需要分配的内存大小
/// align: 内存对齐要求
/// 返回值: 分配的内存块的指针
///
/// |-- BlockHeader (16 bytes) --|- Padding -|-- 用户数据 (size bytes) --|
/// ^                            ^           ^
/// 块头地址 (HEAP_PTR)           对其填充     用户数据地址 (HEAP_PTR + 16)
///
/// 当前 BlockHeader 会默认进行对齐处理，所以当前在对齐时需要满足 BlockHeader 的对齐要求，所以真实使用的对齐大小不可以小于 BlockHeader 的对齐大小。
///
pub fn alloc_from_heap(size: usize, align: usize) -> *mut BlockHeader {
    unsafe {
        if !HEAP_INIT {
            init_heap();
        }
    }

    core::ptr::null_mut()
}
