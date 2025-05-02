import time
st = time.time()
a = 1
b = 1
c = 0
for _ in range(90):
    c = a+b
    a=b
    b=c
ed = time.time()
print(c)
print(ed-st)
# 29.56µs