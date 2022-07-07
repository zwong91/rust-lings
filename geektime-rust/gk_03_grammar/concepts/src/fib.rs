// 控制流程  if 和 loop / while / for
// 计算斐波拉契数列(兔子数列)中下一个数的代码在三个函数中不断重复。这不符合 DRY（Don’t Repeat Yourself）原则, 抽离出来函数
// TODO: O(1) 实现  0、1、1、2、3、5、8、13、21、34 

use firestorm::{bench, profile_fn};
//递归方法
fn fib_recurive(n: i32) -> i32 {
    profile_fn!(fib_recurive);
    match n {
        0 => return 0,
        1 => return 1,
        _ => return fib_recurive(n - 1) + fib_recurive(n - 2),
    };
}

fn fib_loop(n: u8) {
    profile_fn!(fib_loop);
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    println!("\nfib_loop:");

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    profile_fn!(fib_while);
    let (mut a, mut b, mut i) = (1, 1, 2);

    println!("\nfib_while:");

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    profile_fn!(fib_for);
    let (mut a, mut b) = (1, 1);

    println!("\nfib_for:");

    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}

fn test_recurive() {
    fib_recurive(20);
}


fn main() {
    let n = 10;
    bench("./", test_recurive).unwrap();

    fib_loop(n);
    fib_while(n);
    fib_for(n);
}

