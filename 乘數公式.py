import os

n=input('input n= ')
k=input('input k= ')
n=int(n)
k=int(k)

b=n-k

m=a=1
for i in range(n):
    a=a*(m)
    m=m+1
N=n

m=a=1
for i in range(k):
    a=a*(m)
    m=m+1
K=k

m=a=1
for i in range(b):
    a=a*(m)
    m=m+1
B=b

p=float(p)
c=float(c)

p=N/K*B
c=N/K

print(f'P:{p},C:{c}')

os.system('pause')