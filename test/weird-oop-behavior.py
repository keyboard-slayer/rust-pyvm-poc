#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from libpyreval import reval

class A:
    def b(self, msg):
        print("local: ", locals())
        print("global: ", globals())
        print(msg)

def foo(obj):
    print(obj)
    obj("hi")

d = A()
c = d.b

reval(compile("foo(c)", "", "eval"), None, None)
