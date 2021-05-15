import os

n=input('input ')
n=int(n)

def factorial(n):
    a=1
    for i in range(n):
        a=a*n
        n=n-1
    return a

print(factorial(n))
os.system('pause')