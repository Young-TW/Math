import os

m=a=1
n=input('input ')
n=int(n)

for i in range(n):
    a=a*m
    m=m+1

n=a
print(n)
os.system('pause')