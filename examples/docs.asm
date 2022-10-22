; commnets starts with `;` and only works on single lines which doesn't have any operation.

; pushes a float/integer to the stack.
push 0
push 1
push 2
push 3
push 4
push 5

; `dup` jumps <operand> index up in the stack and duplicates the value.
dup 1

; `pop` pops out signle value from stack.
pop

; `add` pops previous two item from stack and push their sum onto the stack.
; the `sub`, `mul` and `div` works the same way.
add

; `out` outputs top most item (as integer) of the stack but doesn't pop it. `outf` does the same. but outputs float
out

; `swap` pops previous one item from stack and swapes it with <operand>.
swp 3

; `eql` pops previous two item from stack and push their 0 or 1 based on wheather they're equal or not onto the stack after. 

; `dump` outputs the whole stack.
dmp

; loop is just an orbitary label name (gotta start with `.` and end with `:`). it's like a varible's name.
.loop:
    ; `jmpif` jumps to the label `loop` if stack's top most item is 0.
    jmpif loop
    ; `jmp` jumps to the label `loop`.
    ; for this specific case, this jump will create an infinite loop
    jmp loop

; `halt` stops execution.
halt