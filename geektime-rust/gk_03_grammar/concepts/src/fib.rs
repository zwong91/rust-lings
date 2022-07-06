// 控制流程  if 和 loop / while / for
// 计算数列中下一个数的代码在三个函数中不断重复。这不符合 DRY（Don’t Repeat Yourself）原则, 抽离出来函数
fn fib_loop(n: u8) {
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
    let (mut a, mut b) = (1, 1);

    println!("\nfib_for:");

    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}

fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}
