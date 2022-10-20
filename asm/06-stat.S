# Get stat() of a file and print file size

# How to get the offset of a struct field?

# One: compile a C application into asm:
/*
 #include <stdio.h>
 #include <sys/stat.h>
 int main() {
    struct stat st;
    printf("%d\n", st.st_size);
}
*/
# -> movq    48(%rsp), %rsi

# Two: compile it using offsetof()
/*
 #include <stdio.h>
 #include <sys/stat.h>
 #include <stddef.h>
 
 int main() {
     printf("%lu\n", offsetof(struct stat, st_size));
 }
*/
# -> 48

# Three: In GDB:
# (gdb) ptype /o MyType

# So our field is at offset 48


.data

# Note: a file is pre-processed when its extension is ".S"
# Buffer size for reading filename from stdin
#define STDIN_READ_BUFFER_SZ 128

# struct stat: struct size, and `st_size` field offset
.set STRUCT_STAT_SZ, 144
.set STRUCT_STAT_FIELD_SIZE, 48


usage_str: 
    .string "Usage: %s filename\n       %s -\n"
enter_filename_str:
    .string "Enter the filename: "
file_size_fmt: 
    .string "File size: %u\n"
file_name_fmt:
    .string "File name: %s\n"
file_name_srdin:
    .string "-"

.text
.global main

main: 
    # Remember stack (will be restored before `ret`)
    push %rbp
    mov %rsp, %rbp

    # When we define `main`, it's a C function.
    # Parameters are passed as usual: %rdi = argc, %rsi = (char**)argv
    # That is, %rdi is the arg count, %rsi points to the beginning of an array of pointers!

    # Check how many arguments.
    # If not two -- print usage. 
    cmpq $2, %rdi
    jne print_usage

    # Read filename: argument or stdin -> %rax
    call get_input_file_name

    # Print the filename
    mov %rax, %rbx  # stash
    mov %rax, %rdi
    call print_file_name

    # Get and print file size
    # Returns: %eax=0 when ok, non-0 when failed
    mov %rbx, %rdi
    call print_file_size
    
    # Restore stack
    mov %rbp, %rsp
    pop %rbp 

    # Exit with code %eax from last func
    ret


# When no arguments are given
print_usage:
    # %rsi: application name
    mov (%rsi), %rax

    # printf() usage
    mov $usage_str, %rdi 
    mov %rax, %rsi 
    mov %rax, %rdx 
    call printf 
    
    # exit(255)
    mov $255, %rdi 
    call exit


get_input_file_name:
    # Enter stack
    push %rbp
    mov %rsp, %rbp

    # Get argv: filename -> %rax
    call get_filename_from_arg
    mov %rax, %rbx

    # Check argument: if it is '-', read from stdin
    mov $file_name_srdin, %rdi
    mov %rbx, %rsi 
    call strcmp
    cmp $0, %rax
    je 2f
    
1:  # %rbx != '-': move value to %rax
    mov %rbx, %rax
    jmp 3f  # skip read from stdin
2:  # %rbx == '-': read from stdin -> %rax
    call read_filename_from_stdin  
3:

    # Leave
    leave
    ret 


# char* get_filename_from_arg()
# Get the input filename from arguments
get_filename_from_arg:
    # Enter stack
    push %rbp
    mov %rsp, %rbp

    # Get filename
    mov 8(%rsi), %rax  # argv[1]

    # Leave
    leave
    ret 



# char* read_filename_from_stdin(int *is_eof)
# Reads a string from user input (stdin)
# NOTE: allocates memory for the buffer!
read_filename_from_stdin:
    # Remember stack (will be restored before `ret`)
    push %rbp
    mov %rsp, %rbp

    # Get some memory
    # TODO: in current implementation, this memory is never deallocated. I'm lazy.
    sub $16, %rsp  # stack alignment ; also, reserve memory for one variable
    mov $STDIN_READ_BUFFER_SZ, %rdi 
    call malloc  # memory address -> %eax
    mov %rax, -8(%rbp)  # pointer to malloc'ed space

    # Prompt
    mov $enter_filename_str, %rdi 
    call printf
    mov $0, %rdi
    call fflush  # line buffering won't output a string before there's an \n. Flush it.

    # Call read()
    mov $1, %rdi  # stdin 
    mov -8(%rbp), %rsi  # pointer to the buffer
    mov $STDIN_READ_BUFFER_SZ, %rdx
    call read

    # Replace any \n with \0
    # This is because user input ends with \n, and stat() won't like it.
    mov -8(%rbp), %rcx
    
0:  # loop start
    # Compare the current symbol to '\0', quit the loop if equal
    cmp $0, (%rcx)
    je 9f

    # Compare the current symbol to '\n'
    cmp $10, (%rcx)  # 10 = '\n'
    jne 2f
1:  # char == '\n'
    mov $0, (%rcx)  # replace it with \0
    jmp 9f
2:  # char != '\n' 
    # next character, loop
    inc %rcx 
    jmp 0b
9:

    # Result -> %rax
    mov -8(%rbp), %rax

    # Restore stack
    mov %rbp, %rsp
    pop %rbp 
    ret 


# Output the file name
# Input:
#   %rdi: filename
print_file_name:
    # Enter stack
    push %rbp
    mov %rsp, %rbp

    # Argument
    mov %rdi, %rax

    # Print the file name
    mov $file_name_fmt, %rdi 
    mov %rax, %rsi 
    call printf

    # Done
    leave
    ret


# print_file_size(char* filename)
# Get the file size by name.
# Args:
#   %rdi: file name
print_file_size:
    # Enter stack
    push %rbp
    mov %rsp, %rbp

    # Argument
    mov %rdi, %rbx

    # Call stat(path, buffer)
    # Allocate memory for `struct stat` (144 bytes)
    # It's already aligned by 16 bytes (144 = 0x90)
    sub $STRUCT_STAT_SZ, %rsp
    mov %rbx, %rdi 
    mov %rsp, %rsi
    call stat

    # Check stat result
    cmp $0, %rax 
    je 1f
0:  # %rax != 0: fail
    # perror(filename), exit(1)
    mov %rbx, %rdi
    call perror
    
    # Retun value: 1 (error)
    mov $1, %rax
    jmp 9f
1:  # %rax == 0: ok   

    # printf() file size
    mov $file_size_fmt, %rdi 
    mov STRUCT_STAT_FIELD_SIZE(%rsp), %rsi
    call printf
    
    # Result ok
    mov $0, %rax

9:
    # Done
    leave
    ret