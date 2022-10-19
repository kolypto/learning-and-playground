# Shows how to call printf

.global main

.text 
main:
    # We call printf, which may be using eax/ebx/ecx registers and will remove our data.
    # So we will save these to stack before the call and restore them afterwards

    # Calling convention.
    # To call printf, we need to use calling conventions for 64bit code
    # * Integer arguments: rdi, rsi, rdx, rcx, r8, r9
    # * Floating-point arguments: xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7
    # * Additional parameters are pushed on the stack, left to right, and are removed after the call.
    # * After the call, the return address is at %rsp, the first memory parameter is at 8(%rsp)
    # * !!! The stack pointer %rsp must be aligned to a 16-byte boundary before making a call!!
    # * The function is not allowed to modify the calle-save registers: rbp, rbx, r12, r13, r14, r15. 
    # * All other registers are free to be changed by the function.
    # * Integers are returned in %rax or %rdx:%rax. Floating-point values are returned in xmm0 or xmm1:xmm0.
    # 

    # This is called by the C library startup code
    # We do this to make sure that the stack is 16-byte aligned.
    # Here, we assume that the stack is aligned before the call. 
    # Why align? 
    #   Because processors can efficiently access memory at the granularity and alignment of their word size.
    #   The CPU always reads at its word size, which is 16 bytes for x86_64.
    #   So when you do an unaligned address access (on a processor that supports it) -- 
    #   the processor is going to read multiple words.
    #   This causes an amplification of up to 2X the number of memory transactions required.
    # How to check?
    # The value of RSP before calling any function has to be in the format of 0xXXXXXXXXXXXXXXX0.
    # Otherwise, it may segfault!
    push    %rbx
    
    # Save registers
    push %rax   # caller-save register
    push %rcx   # caller-save register 

    # Call printf
    # Stack is already aligned because we pushed three 8-byte registers
    mov $format, %rdi   # 1st parameter
    mov $31337, %rsi    # 2nd parameter
    xor %rax, %rax      # because printf is varargs
    call printf 

    # Check ret code, say "formatting error" if it went wrong
    cmp $0, %rax
    jg format_error

    # Restore registers after the call
    pop %rcx 
    pop %rax

    # Restore %rbx for Linux
    # And return from main back into C library wrapper
    pop %rbx
    ret 


format_error:
    # write() syscall
    mov $1, %rax  # syscall: 1=write
    mov $1, %rdi  # 1 = stdout
    mov $format_error_str, %rsi  # string 
    mov $format_error_str_len, %rdx  # strlen
    syscall 

    # exit(255) syscall
    mov $60, %rax  # exit()=60
    mov $255, %rdi
    syscall

format:
    .string "%20ld\n"
format_error_str:
    .string "Formatting error\n"
    .set format_error_str_len, . - format_error_str - 1
