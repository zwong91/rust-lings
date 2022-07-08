// 在 Rust 下，函数是一等公民，可以作为参数或者返回值(闭包), fn(i32) -> i32 这个类型是一个函数指针类型
// https://doc.rust-lang.org/reference/types/function-pointer.html
/* 函数本身是一个指向 TEXT 段起始位置的指针。我们可以看到它可以 cast 成 *const () 。
```rust
fn main() {
 println!("main: {:p}", main as *const ());
}
```
*/

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}
/*
缺省返回值 unit类型
*/
fn main() {
    println!("main: {:p}", main as *const ());
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));
}
