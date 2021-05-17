import os
n=input('input ')
n=int(n)

a=1
def fact(n):
    if n == 0:
        return 1
    else:
        global a
        a=a*n
        return fact(n-1)

fact(n)
print(a)
os.system('pause')
