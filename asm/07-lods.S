.data
    sample_string_with_spaces: 
        .string "Sa mp le st ri ng wi th sp ac es!\n"

.text 
.global main 
main:
    # lods: Loads element into $rax and increases $rsi by 1 word (depending on the command size)
    # stos: Saves %rax into %rdi and increases %rdi
    
    # Enter stack
    push %rbp
    mov %rsp, %rbp
    
    #sub $8, %rsp  # alignment

    # Get str length
    mov $sample_string_with_spaces, %edi 
    call strlen 

    # Allocate as many bytes
    mov %rax, %rdi 
    call malloc

    # Remember this memory address
    mov %rax, %rbx

    # Remove every space
    mov $sample_string_with_spaces, %rsi
    mov %rax, %rdi
    
    # Iterate: use `lods` to load bytes into %rax
z:
0:  
    # Load character
    lodsb

    # Skip spaces
    cmpb $32, %al  # ' ' = $32
    je 0b

    # Save character
    stosb

    # Keep iterating
    cmpb $0, %al
    jne 0b
1:

    # printf()
    mov %rbx, %rdi 
    call printf

    # Done
    mov $0, %eax
    leave
    ret
