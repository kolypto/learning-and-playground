# Это 32хбитный код
.code32

# Поместить следующее в сегмент данных
.data  

hello_str:
        # Строка и её длина
        .string "Hello, world!\n"
        .set hello_str_length, . - hello_str - 1
 
# Поместить в сегмент кода
.text       
 
# main - глобальный символ, видимый за пределами текущего файла. 
# Это функция, а не данные.
.globl  main 
.type   main, @function   
 
# Код функции
main:
        # Вызов функции write()
        # %eax = номер системного вызова
        # %ebx, %ecx, %edx - параметры функции
        # int 0x80: вызов функции через прерывание
        movl    $4, %eax                # поместить номер системного вызова write = 4 в регистр %eax       
        movl    $1, %ebx                # первый параметр - в регистр %ebx; номер файлового дескриптора stdout = 1                        
        movl    $hello_str, %ecx        # второй параметр - в регистр %ecx; указатель на строку            */
        movl    $hello_str_length, %edx # третий параметр - в регистр %edx; длина строки  
        int     $0x80                   # вызвать прерывание 0x80: syscall (legacy way)       
 
        # Вызов фукнции exit()
        movl    $1, %eax      # номер системного вызова exit = 1   
        movl    $0, %ebx      # передать 0 как значение параметра  
        int     $0x80         # вызвать exit(0)                    
 
        
        .size   main, . - main   # размер функции main           


# Compile:
# With `as`:
#   $ as 01-helloworld.s -o 01-helloworld.o
#   $ ld 01-helloworld.o -o 01-helloworld
# Or with `gcc`:
#   $ gcc -no-pie 01-helloworld.s -o 01-helloworld
# Or compile with debugging information:
#   $ gcc -no-pie -g 01-helloworld.s -o 01-helloworld
