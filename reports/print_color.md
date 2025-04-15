# 解题思路

![alt text](../images/1.png)

1. 修改[sbi::putchar](../arceos/modules/axhal/src/platform/riscv64_qemu_virt/console.rs)的内容, 添加颜色信息. 

```rust
/// Writes a byte to the console.
pub fn putchar(c: u8) {
    // #[allow(deprecated)]
    // for b in b"\x1b[31m" {
    //     sbi_rt::legacy::console_putchar(*b as usize);
    // }
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c as usize);
    // #[allow(deprecated)]
    // for b in b"\x1b[31m" {
    //     sbi_rt::legacy::console_putchar(*b as usize);
    // }
}
```


2. 修改[marcos](../arceos/ulib/axstd/src/macros.rs)的内容, 添加颜色信息. 

```rust
/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!("\x1b[31m{}\x1b[0m\n", format_args!($($arg)*)));
    }
}
```




3. 修改[main.rs](../arceos/exercises/print_with_color/src/main.rs)的内容,直接在输出结果上添加颜色信息 .

```rust
fn main() {
    println!("\x1b[31m[WithColor]: Hello, Arceos!\x1b[0m");
}
```