/*
「在编译时，一切无法确定大小或者大小可以改变的数据，都无法**安全地**放在栈上，**最好**放在堆上」

gcc 的实现将可变参数放在栈上（估计是为了性能） 会导致访问栈上的垃圾内容，甚至导致程序崩溃

alloca() 可以在栈上分配动态大小的内存, 官方建议配合 longjmp 使用
*/
#include <stdio.h>
#include <stdarg.h>

int sum(int count, ...) { 
    va_list ap;
    int i;
    double sum = 0;

    va_start(ap, count);
    for (i = 0; i < count; i++)
    {
        sum += va_arg(ap, int);
    }
    
    va_end(ap);

    return sum;
 }

int main(int argc, char **argv) {

    printf("%d\n", sum(10, 1, 2, 3)); // 传入 3 个值但 count 为 10
    return 0;
}