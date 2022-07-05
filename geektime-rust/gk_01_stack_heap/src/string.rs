fn main() {
    let data: String = "hello world".into();

    let s1: &str = &data;
    let s2: &str = &data;
    let s3: &str = &data;
    // "{:p}"输出某个变量的地址，是这个变量自身结构的地址
    dbg!(&s1 as * const _);
    dbg!(&s2 as * const _);
    dbg!(&s3 as * const _);
    // "{:p}"输出某个变量的地址，是变量所指向值的地址
    dbg!(&s1.as_bytes() as * const _);
    dbg!(&s2.as_bytes() as * const _);
    dbg!(&s3.as_bytes() as * const _);

}