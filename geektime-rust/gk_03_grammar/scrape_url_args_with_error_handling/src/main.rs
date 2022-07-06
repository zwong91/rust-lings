// 从你第一个Rust程序开始  ./scrape_url_args_with_error_handling  https://www.rust-lang.org  rust.md
// Rust 是一门基于表达式（expression-based）的语言, 除了 let / static / const / fn 等少数语句外，Rust 绝大多数代码都是表达式
// 很多语言的 while loop，for loop，if else 都是语句，但 Rust 下它们是表达式，它们会返回最后一个表达式的值（即便是 unit）
// Rust 既有statement又有expression，Rust的statement主要用于显式顺序表达式求值以及包含一些表达式 https://doc.rust-lang.org/reference/statements-and-expressions.html

// Rust 通过 mod/crate/workspace 组织代码
/* Rust 支持声明宏（declarative macro）和过程宏（procedure macro），其中过程宏
又包含三种方式：函数宏（function macro），派生宏（derive macro）和属性宏
（attribute macro）

println! 是函数宏, Rust 是强类型语言，函数的类型需要在编译期敲定，而 println! 接受任意个数的参数，只能用宏来表达
*/
use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for arg in std::env::args() {
        // println! Rust 函数式过程宏, 支持可变长参数
        println!("{}", arg);
    }
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        println!("Usage: scrape_url_args <url> <output file>");
        std::process::exit(1);
    }

    let url = &args[0];
    let output = &args[1];

    println!("Fetching url: {}", url);
    // Result<T, E>  ?错误传播
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);


    Ok(())
}
