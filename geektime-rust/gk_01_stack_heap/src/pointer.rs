/**
 * 内存：变量和值存放位置, 值放堆上还是放栈上，这是一个问题
 * 在编译时，一切无法确定大小或者大小可以改变的数据，都无法安全
地放在栈上，最好放在堆上
调用栈的大小，避免栈溢出（stack overflow）

FP 是 frame pointer，指向栈帧的起始位置。还有一个 SP（stack pointer）指向栈
顶，它会随着栈的操作而变化。在开一个新的栈帧的时候，在栈顶会压入 callee 函数的参数，cal
ler 返回地址，以及 caller FP


当我们需要动态大小的内存时，只能使用堆
动态生命周期的内存也需要分配到堆上
堆上内存有更加灵活的生命周期，可以在不同的调用栈之间共享数据

第一大内存安全问题:
一个线程在遍历列表，而另一个线程在释放列表中的某
一项，就可能访问野指针，导致堆越界（heap out of bounds）

第二大内存安全问题:
堆上内存被释放，但栈上指向堆上内存的相应指针没有被清空，就有可能发生使用已
释放内存（use after free）的情况

Q & A: 
1. 如果有一个数据结构需要在多个线程中访问，可以把它放在栈上吗？为什么？ Cross-thread Stack Access
2. 可以使用指针引用栈上的某个变量吗？如果可以，在什么情况下可以这么做？

1. 在多线程场景下，每个线程的生命周
期是不固定的，无法在编译期知道谁先结束谁后结束，所以你不能把属于某个线程 A 调用栈上的
内存共享给线程 B，因为 A 可能先于 B 结束。这时候，只能使用堆内存。这里有个例外，如果结
束的顺序是确定的，那么可以共享，比如 scoped thread；

每个线程有独立的栈和寄存器, 栈上数据无法进行跨栈共享访问, 一旦该函数调用返回后, 栈的内存就会被回收

2. 而同一个调用栈下，main() 调用 hello()，再调用 world()，编译器很清楚，world() 会先结束，之后是 hello()，最后是 main()。
所以在 world() 下用指针引用 hello() 或者 main() 内部的变量没有问题，这个指针必然先于它指向
的值结束。这个两个问题的实质是我们要搞明白哪些东西在编译期可以确定它们的关系或者因
果，哪些只能在运行期确定。
只要指针的生命周期小于或者等于栈上的引用源就行,如果生命周期大于引用源就会出
现野指针的情况

*/
static MAX: u32 = 0;

fn foo() {
    let hello = "hello world";
    println!("foo RODATA: {:p}", hello);
}

fn bar() {
    let hello = "hello world";
    println!("bar RODATA: {:p}", hello);
}

fn main() {
    // 字符串常量string literals 在编译时被存入可执行文件的指向RODATA段, 在程序加载时，获得一个固定的内存地址
    let hello = "hello world";
    println!("RODATA: {:p}", hello);
    // foo() 和bar()中，各自都用到了一个string literal "hello world", 
    // 那么编译器从从可执行文件.rodata 中加载内存, 加载一份"hello world" 它们指向 .rodata 中同样的地址
    foo();
    bar();

    // 在堆上，一块新的内存被分配出来，并把 helloworld 逐个字节拷贝过去 memcpy
    //胖指针（fat pointer） ptr len  cap
    let hello = "hello world".to_string();


    let data = Box::new(1);

    // static 执行 Data Section
    println!("DATA (static var): {:p}_", &MAX);

    // function 在TEXT
    println!("TEXT:(FUNCTION: {:p}_",  foo as *const () );

    // String 结构体分配在栈上, 其引用指向一个栈地址
    print!("STACK (&helllo): {:p}", &hello);
    
    // 通过解引用获取其堆上数据,  然后取其引用
    println!("HEAP: (&*hello){:p}", &*hello);

    // Box 实现了 Pointer trait 无需额外的解引用
    println!("HEAP (box impl Pointer) {:p}, {:p}", data, &*data);
}

