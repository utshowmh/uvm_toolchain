; calculates fibonacci numbers.

push 0
push 1
push 0
.loop:
    pop
    dup 1
    out
    dup 1
    add
    dup 0
    push 1000000000
    geql
    not
    jmpif loop
hlt