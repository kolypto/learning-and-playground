# Passing arguments to functions (subprograms)

.data

number_fmt: .string "Result: 0x%04x\n"

.text
.global  main 
.type   main, @function   
main:
    # call <label>
    # = jmp <label>, and also put the next instruction address into the stack
    # ret [num]
    # = jmp (%eip), and also increase %esp by <num> (optional): to remove own arguments from the stack

    # Passing arguments:
    # Via registers: very fast, but few
    # Via shared mem: not thread safe, slow. Don't use.
    # Via stack

    # Pass arguments via stack:
    push $0x10
    push $0x20
    push $0x30
    call do_something_with_stack

    # Pass arguments via registers (already in `eax`)
    call print_number

    ret 

# Example function that receives arguments via registers
# print(): eax = the number to print
print_number:
    mov $number_fmt, %rdi 
    mov %rax, %rsi 
    call printf
    ret


# Example function that receives arguments via stack
# Three arguments
do_something_with_stack:
    # Remember the current %ebp; then temporarily use it to keep the current stack ptr
    push %rbp
    mov %rsp, %rbp  # remember the current stack state

    # Reserve some space for local variables in the stack.
    # This is faster than `push` because it does not write to memory. Only modifies the register.
    sub $8, %rsp 

    # Get parameters #1, #2, #3
    # Add them
    xor %rax, %rax
    add (8*4)(,%rbp), %rax  # first
    add (8*3)(,%rbp), %rax  # second
    add (8*2)(,%rbp), %rax  # third

    # Restore stack
    mov %rbp, %rsp  # Let it point to the same 
    pop %rbp 
    ret $(3*8)  # drop these 3 parameters we've been given
