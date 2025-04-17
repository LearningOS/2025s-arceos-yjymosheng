# 题目思路

非常简单, 因为只有[lib.rs](../arceos/modules/bump_allocator/src/lib.rs)需要修改

# 题目难点

1. Byte 分配需要关注对齐 . 对齐公式 通过gpt分享得知 .具体的推导我仅仅手动算了一遍

2. Page 分配需要理解num_pages 和 align_pow2的概念 . 

    首先一个是分配的页数,这个不太困难; align_pow2,这个通过查询资料可知 是页面的对齐. 它跟Byte
    的对齐有什么区别呢? 毕竟有page_size这种天然的对齐标准. gpt提供了关于dma组建的对齐示例, 也
    就是os 可能需要对齐更大的页

3. 对AllocResult 的理解 ,需要针对不同的情况返回不同的Err值

```rust
/// The error type used for allocation.
#[derive(Debug)]
pub enum AllocError {
    /// Invalid `size` or `align_pow2`. (e.g. unaligned)
    InvalidParam,
    /// Memory added by `add_memory` overlapped with existed memory.
    MemoryOverlap,
    /// No enough memory to allocate.
    NoMemory,
    /// Deallocate an unallocated memory region.
    NotAllocated,
}

/// A [`Result`] type with [`AllocError`] as the error type.
pub type AllocResult<T = ()> = Result<T, AllocError>;
```