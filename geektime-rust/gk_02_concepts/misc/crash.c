#include "stdio.h"

void hello() {
    printf("Hello world!\n");
}

// 函数放在代码段中，通常是只读的，往只读段写入数据会触发保护异常
// C 语言中，这种操作是允许的，但会造成内存访问越界，导致崩溃, 在访问列表，或者修改列表的时候会挂掉
// 把一个函数指针解引用为一个列表的，往列表中插入一个元素会报错, 函数是`代码`，在内存中通常会存放在只读区域.text段，对这部分内存写会报错 aligment 错误  segment fault
int main() {
    char buf[1024];
    void (* p)() = &hello;
    (*p)();
    // 转成数组指针
    int *p1 = (int *) p;
    p1[1] = 0xdeadbeef;
}
