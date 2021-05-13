import os
n=input('input ')

def factorial(n):
    m=a=1
    n=int(n)

    for i in range(n):
        a=a*(m)
        m=m+1

    n=a
    print(n)
    os.system('pause')

factorial(n)