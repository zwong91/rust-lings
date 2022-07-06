#!/usr/bin/python
def add_numbers(a, b):
    return a + b

# Python 会做类型的隐式转换
if __name__ == '__main__':
    a = 42
    b = 42.0
    print(add_numbers(a,b))