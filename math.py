import math
from os import system

#遞迴階乘
def fact():
    n=input('input ')
    n=int(n)

    a=1
    def f(n):
        if n == 0:
            return 1
        else:
            global a
            a=a*n
            return f(n-1)

    f(n)
    print(a)
    system('pause')

#加減乘除
def plus():
    a=input('input n1=')
    a=int(a)
    b=input('input n2=')
    b=int(b)
    c = a + b
    print(c)
    system('pause')

def minus():
    a=input('input n1=')
    a=int(a)
    b=input('input n2=')
    b=int(b)
    c = a - b
    print(c)
    system('pause')
    
def multiplication():
    a=input('input n1=')
    a=int(a)
    b=input('input n2=')
    b=int(b)
    c = a * b
    print(c)
    system('pause')

def division():
    a=input('input n1=')
    a=float(a)
    b=input('input n2=')
    b=float(b)
    c = a / b
    print(c)
    system('pause')

#開方
def power():
    a=input('input n1=')
    a=int(a)
    b=input('input n2=')
    b=int(b)
    c=a**b
    print(c)

#最小公倍數
def lcm():
    n=input('input n1=')
    n=int(n)
    m=input('input n2=')
    m=int(m)

    if n>m:
        c=n
    else:
        c=m

    while 1:
        if c%n==0 and c%m==0:
            print(f"Least common multiple:{c}")
            break
        else:
            c=c+1	
    		
    system('pause')

string = input('')

