#!/usr/bin/env python3

import time
from libpyreval import reval

def fib(n):
    start = [0, 1, 0]
    
    print(0)
    print(1)

    for _ in range(n):
        start[-1] = start[0]
        start[0] += start[1]
        start[1] = start[-1]
        print(start[0])
    

print(" --- NATIVE --- ")
start = time.time()
eval(compile("fib(20)", "", "eval"))
print(f" --- {time.time()-start} SEC --- \n")
print(" --- REVAL ---")
start = time.time()
reval(compile("fib(20)", "", "eval"), None, None)
print(f" --- {time.time()-start} SEC --- \n")
