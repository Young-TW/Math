import os

def fact(x): 
    m=a=1
    for i in range(x):
        a=a*(m)
        m=m+1
    return a
    

n=input('input n= ')
k=input('input k= ')
n=int(n)
k=int(k)

b=n-k

fact(n)
print(n)
fact(k)
print(k)
fact(b)
print(b)
p=n/(k*b)
c=n/k

print(f'P:{p},C:{c}')

os.system('pause')