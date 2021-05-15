import os
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

os.system('pause')