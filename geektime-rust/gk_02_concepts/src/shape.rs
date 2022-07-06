/*
数据- 程序操作的对象： 值、类型、指针、引用
代码- 程序运行的主体: 函数、方法、闭包、接口、虚表
运行方式- 程序执行效率：并发、并行、同步、异步
编程范式- 提升代码质量: 泛型编程

类型对内存中值的长度、对齐以及值可以进行的操作
等信息， 值是无法脱离具体的类型讨论的


所有原生类型的大小都是固定的，因此它们可以被分配到栈上

指针和引用是原生类型，它们可以分配在栈上

指针通过解引用（deference）来访问它指向的内存地址, 引用的解引用访问是受限的，它只能解引
用到它引用数据的类型，不能用作它用

函数也是对代码中重复行为的抽象闭包引用的上下文中的
自由变量，会被捕获到闭包的结构中，成为闭包类型的一部分

作为一个抽象层，接口将使用方和实现方隔离开来，使两者不直接有依赖关系，大大提高了复用
性和扩展性

在运行时，一旦使用了关于接口的引用，变量原本的类型被抹去
在生成这个引用的时候，我们需要构建胖指针，除了指向数据本身外，还需要指向
一张涵盖了这个接口所支持方法的列表(虚表)

代码以何种方式运行，往往决定着程序的执行效率 async/await（syntactic sugar） Promise将来时态，从未来的某个时刻才能获得的结果的值
 CPU、内存、I/O 设备、和网络的延迟，遭遇 I/O 处理时，高效 CPU 指令和低效 I/O 之间的巨大鸿沟,  异步IO


虚表相当于在运行时生成的一个涵盖了一系列函数指针的数据结构。有时候对于不同类
型但是满足相同接口的数据，我们希望可以抹去他们的原始类型，让它们有相同的接口类型，以
便于统一处理，这样更加灵活，但此时需要为每个数据构造他们各自对接口实现的虚表，这样可
以依旧调用到属于该类型的实现
虚表一般存储在堆上, Rust 有栈实现https://github.com/archshift/dynstack


数据结构的泛型 在“调用”时，它接受若干个使用
了具体类型的参数，返回携带这些类型的类型

泛型编程通过参数化让数据结构像函数一样延迟绑定，提升其通用性，类型的参数可以用
接口约束，使类型满足一定的行为


Q & A:
1. 有一个指向某个函数的指针，如果将其解引用成一个列表，然后往列表中插入一个元
素，请问会发生什么？（对比不同语言，看看这种操作是否允许，如果允许会发生什么）
对于强类型语言（如：rust），无法解引用成一个列表，rust会提示类型不匹配错误。
对于弱类型语言(如：python, javascript)，解引用成一个列表后，可以正常插入元素 定义成有虚函数的基类

2. 要构造一个数据结构 Shape，可以是 Rectangle、 Circle 或是 Triangle，这三种结构见
如下代码。请问 Shape 类型该用什么数据结构实现？怎么实现？
struct Rectangle { a: f64, b: f64, }
struct Circle { r: f64, }
struct Triangle { a: f64, b: f64, c: f64, }
3. 对于上面的三种结构，如果我们要定义一个接口，可以计算周长和面积，怎么计算？

*/

const MY_PI: f64 = std::f64::consts::PI;

struct Rectangle{
    a: f64,
    b: f64,
}

struct Circle {
    r: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}


#[warn(dead_code)]
enum Shape {
    Rec(Rectangle),
    Cir(Circle),
    Tri(Triangle),
}

trait Area {
    fn area(&self) -> f64;
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rec(r) => r.area(),
            Shape::Tri(t) => t.area(),
            Shape::Cir(c) => c.area(),
        }
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.r * self.r * MY_PI
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        let (a, b, c) = (self.a, self.b, self.c);
        let p = (a + b + c ) / 2.0;

        (p * (p -a) * (p - b) * (p - c)).sqrt()
    }
}

fn main() {

    let rec = Rectangle{
        a: 1f64,
        b: 2f64,
    };

    let cir = Circle{
        r: 10f64,
    };

    let tri = Triangle{
        a: 1f64,
        b: 2f64,
        c: 3f64,
    };

    let shape = Shape::Rec(rec);
    println!("area is {}", shape.area());
}