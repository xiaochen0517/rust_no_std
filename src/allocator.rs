use core::alloc::GlobalAlloc;

use crate::println;

static mut HEAP: [u8; 1024 * 1024] = [0; 1024 * 1024];
static HEAP_SIZE: usize = 1024 * 1024; // 1 MiB
static mut HEAP_INIT: bool = false;
static mut HEAP_START: usize = 0;
static mut HEAP_PTR: usize = 0;

///
/// 已分配的内存块头结构
///
/// size: 用户数据的大小
/// is_free: 是否空闲
///
#[repr(C)]
pub struct BlockHeader {
    size: usize,   // 8 bytes
    is_free: bool, // 1 byte, 7 bytes padding
} // Total: 16 bytes

impl BlockHeader {
    ///
    /// 从用户数据指针获取块头结构指针
    ///
    unsafe fn from_user_data_ptr(user_data_ptr: *mut u8) -> *mut BlockHeader {
        let header_size = core::mem::size_of::<BlockHeader>();
        let header_ptr = (user_data_ptr as usize - header_size) as *mut BlockHeader;
        header_ptr
    }

    ///
    /// 从块头结构指针获取用户数据指针
    ///
    unsafe fn get_user_data_ptr(&self) -> *mut u8 {
        (self as *const BlockHeader as usize + core::mem::size_of::<BlockHeader>()) as *mut u8
    }
}

///
/// 初始化堆内存
///
pub fn init_heap() {
    unsafe {
        if HEAP_INIT {
            return;
        }
        HEAP_START = (&raw const HEAP) as usize;
        HEAP_PTR = HEAP_START;
        HEAP_INIT = true;
    }
}

///
/// 内存对齐函数
///
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

///
/// 从堆中分配内存
/// size: 需要分配的内存大小
/// align: 内存对齐要求
/// 返回值: 分配的内存块的指针
///
/// |- Padding -|-- BlockHeader (16 bytes) --|-- 用户数据 (size bytes) --|
/// ^           ^                            ^
/// 对其填充     块头地址 (HEAP_PTR)           用户数据地址 (HEAP_PTR + 16 + padding)
///
/// 当前 BlockHeader 会默认进行对齐处理，所以当前在对齐时需要满足 BlockHeader 的对齐要求，所以真实使用的对齐大小不可以小于 BlockHeader 的对齐大小。
///
pub fn alloc_from_heap(size: usize, align: usize) -> *mut BlockHeader {
    unsafe {
        if !HEAP_INIT {
            init_heap();
        }
        // 获取当前块头结构的对齐要求
        let header_align = core::mem::align_of::<BlockHeader>();
        let header_size = core::mem::size_of::<BlockHeader>();
        // 取对其参数和块头结构对齐要求的最大值，保证当前块头结构的对齐满足要求
        let align: usize = align.max(header_align);
        // 计算用户数据的起始地址
        let user_data_start = align_up(HEAP_PTR + header_size, align);
        // 检查堆内存是否足够
        if user_data_start + size > HEAP_START + HEAP_SIZE {
            panic!("Out of heap memory");
        }
        // 计算当前块头结构的地址
        let header_ptr = (user_data_start - header_size) as *mut BlockHeader;
        (*header_ptr).size = size;
        (*header_ptr).is_free = false;
        // 更新堆指针
        HEAP_PTR = user_data_start + size;
        // 返回块头结构的指针
        header_ptr
    }
}

///
/// 已释放的空闲内存块结构
///
#[repr(C)]
struct FreeBlock {
    size: usize,          // 8 bytes
    next: *mut FreeBlock, // 8 bytes
} // Total: 16 bytes

///
/// 空闲链表结构
///
struct FreeList {
    head: *mut FreeBlock,
}

impl FreeList {
    const fn new() -> Self {
        FreeList {
            head: core::ptr::null_mut(),
        }
    }

    ///
    /// 将一个块头添加到空闲链表中
    ///
    fn add_free_block(this: *mut Self, block_ptr: *mut BlockHeader) {
        unsafe {
            let free_block_ptr = block_ptr as *mut FreeBlock;
            (*free_block_ptr).next = (*this).head;
            (*this).head = free_block_ptr;
        }
    }

    ///
    /// 在空闲链表中查找一个足够大的块来满足分配请求
    ///
    /// size: 需要分配的内存大小
    /// 返回值: 满足分配请求的块头结构指针
    ///
    fn find_fit(this: *mut Self, size: usize) -> Option<*mut BlockHeader> {
        unsafe {
            // 简单的首次适配算法
            let mut prev: *mut FreeBlock = core::ptr::null_mut();
            let mut current = (*this).head;
            while !current.is_null() {
                // 如果当前空闲块足够大，将其转换为 BlockHeader 并返回
                if (*current).size >= size {
                    // 从空闲链表中移除当前块
                    if !prev.is_null() {
                        (*prev).next = (*current).next;
                    } else {
                        (*this).head = (*current).next;
                    }
                    let block_ptr = current as *mut BlockHeader;
                    (*block_ptr).is_free = false;
                    return Some(block_ptr);
                }
                // 继续遍历空闲链表
                prev = current;
                current = (*current).next;
            }
            None
        }
    }
}

///
/// 全局空闲链表实例
///
static mut FREE_LIST: FreeList = FreeList::new();

///
/// 堆内存分配器实现
///
pub struct MiniAllocator;

unsafe impl GlobalAlloc for MiniAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        println!(
            "Allocating {} bytes with alignment {}",
            layout.size(),
            layout.align()
        );
        unsafe {
            // 先尝试从空闲链表中查找一个满足分配请求的块
            if let Some(block_ptr) =
                FreeList::find_fit(&raw mut FREE_LIST as *mut FreeList, layout.size())
            {
                return (*block_ptr).get_user_data_ptr();
            }
            // 如果空闲链表中没有满足分配请求的块，则从堆中分配一个新的块
            let block_ptr = alloc_from_heap(layout.size(), layout.align());
            (*block_ptr).get_user_data_ptr()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        println!(
            "Deallocating {} bytes with alignment {} at address {:p}",
            layout.size(),
            layout.align(),
            ptr
        );
        unsafe {
            // 获取块头结构指针
            let block_ptr = BlockHeader::from_user_data_ptr(ptr);
            if (*block_ptr).is_free {
                // 如果块已经是空闲的，说明发生了 double free 错误，直接返回
                panic!("Double free detected");
            }
            // 将块头结构添加到空闲链表中
            FreeList::add_free_block(&raw mut FREE_LIST as *mut FreeList, block_ptr);
        }
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: MiniAllocator = MiniAllocator;
