# 解题思路

1. 修改[std](../arceos/ulib/axstd/src/lib.rs)的内容,直接引入rust std hashbrown作为hashmap算法



2. 给axstd[hashmap.rs](../arceos/ulib/axstd/src/hashmap.rs) 手动实现hahsmap的内容

```rust
pub mod hashmap; 
pub use hashmap as collections;
```
**难点** : 

    1. 涉及一些hashmap的原理思想. 图简单,我直接设置了比较大的bucket , 冲突相关的内容也没有多管
    
    2. 需要掌握iter的内容,因为main.rs中有关于iter方法的调用