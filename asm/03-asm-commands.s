.data
    leet: .long 0x31137

    array: .long 0x01, 0x02, 0x03 
    array_end:  # marks the end of the array (!)

    printf_number: .string "%d\n"

.text

.global  main 
.type   main, @function   
main:
    # Reset the register
    xor %eax, %eax

    # === mov
    # Set value of a register
    movl $leet, %eax # put address of $leet
    movl leet, %eax  # put $leet value

    # Get array[1] into %eax
    movl array+4, %eax 

    # Get array[1] into %eax, using %ecx as index register
    movl $1, %ecx
    movl array(,%ecx,4), %eax  # %eax=2

    # Change array[0]: put its pointer into %eax, modify it using "movl"
    movl $array, %eax 
    movl $0xFF, (%eax)  # write into "the address at %eax"



    # === lea
    # > lea <src> <dst>
    # Load effective address. Puts the address of <src> into <dst>
    leal array, %eax  # same as: movl $array %eax



    # === push, pop
    # In Linux ABI, stack is aligned by 4 bytes
    
    # Push two values, move them into %eax and %ebx
    pushq $0x20
    pushq $0x10
    popq %rax  # 0x10
    popq %rbx  # 0x20



    # === math
    # inc <operand>         operand++
    # dec <operand>         operand--
    # add <src>, <dst>      dst += src
    # sub <src>, <dst>      dst -= src
    # mul <operand>         %eax *= operand

    # When byte overflow occurs, `cf` flag is set. Use `jnc` to jump in this case:
    movb $255, %al 
    addb $1, %al
    #jnc overflow



    # === loop
    # loop <label>
    # Works: if (%ecx--) == 0, continues. Otherwise, goto <label>

    # Results will go to %eax. Reset.
    movq $0, %rax

    # loop, 10 steps, sum numbers {0..10}
    movq $10, %rcx
sum:addq %rcx, %rax
    loop sum 

    # printf(). 
    # See 04-asm-printf.s
    push $0     # !! for 16-byte stack alignment.
    mov $printf_number, %rdi
    mov %rax, %rsi
    xor %rax, %rax  # Zeroing EAX is efficient way to clear AL
    call printf 
    pop %rax  # remove the value from the stack



    # === Comparison
    # cmp <op2>, <op1>
    #   Subtracts <op1>-<op2> and sets the flags.
    # je: equal:    jump if equal
    # jn: not:      jump if not equal
    # jg: greater:  jump if op1 > op2, signed
    # jl: less:     jump if op1 < op2, signed
    # ja: above:    jump if op1 > op2, unsigned
    # jb: below:    jump if op1 < op2, unsigned
    # Also: jne, jng, jnl, jna, jnb

    # Task: find the largest element
    # Result: The largest element will be stored here 
    mov array, %eax    # Greatest element value
    mov $array, %ebx   # Current element ptr
loop_start:
    cmp %eax, (%ebx)   # compare the current element
    jle next           # skip next line if the current element is not max
    mov (%ebx), %eax   # remember the greatest value
next:
    add $4, %ebx  # go to next element. 4 = sizeof(long)
check_bounds:
    cmp $array_end, %ebx 
    jne loop_start

    # printf()
    mov $printf_number, %rdi
    mov %rax, %rsi
    push $0  # alignment
    call printf 
    pop %rax



    # === Boolean operators
    # and <src> <dst>
    # or <src> <dst>
    # xor <src> <dst>
    # not <op>
    # test <op1> <op2> -- like `and`, but does not modify operands. It only sets the flags.
    # sal/shl num, <dst>    - shift arithmetic left / shift logical left. Shifted bit is placed into `CF` flag
    # sar/shr num, <dst>    - shift arithmetic right / shift logical right Shifted bit is placed into `CF` flag
    # ror/rol num, <dst>    - cyclic shift (rotation)
    # rcr/rcl num, <dst>    - cyclic shift through `CF` flag (sees `CF` as an extension of the operand)
    testb $0b00001000, %al
    je not_set  # jump here if 3rd bit is not set
not_set:
    xor %eax, %eax  # faster than `mov`, but it sets flags. `mov` does not modify flags.


    # === 



    # Done
    ret
