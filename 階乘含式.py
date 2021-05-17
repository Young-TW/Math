import os
n = None

def factorial(n):
    n=input('input ')
    a=1
    n=int(n)

    for i in range(n):
        a=a*n
        n=n-1

    print(a)
    os.system('pause')

factorial(n)