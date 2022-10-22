; calculates fibonacci numbers 'till inf.
push 0
push 1
.loop:
    dup 1
    dup 1
    add
    out
    jmp loop