static MAX: u32 = 0;

fn foo() {}

fn main() {
    let hello = "hello world".to_string();
    let data = Box::new(1);

    // string literals 指向 RODATA 地址
    println!("RODATA: {:p}", "hello world");
    // static 执行 Data Section
    println!("DATA (static): {:p}_", &MAX);

    // function 在TEXT
    println!("TEXT:(FUNCTION: {:p}_",  foo as *const () );

    // String 结构体分配在栈上, 其引用指向一个栈地址
    print!("STACK (&helllo): {:p}", &hello);
    // 通过解引用获取其堆上数据,  然后取其引用
    println!("HEAP: (&*hello){:p}", &*hello);

    // Box 实现了 Pointer trait 无需额外的解引用
    println!("HEAP (box impl Pointer) {:p}, {:p}", data, &*data);
}