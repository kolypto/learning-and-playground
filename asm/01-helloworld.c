#include <unistd.h>

int main(int argc, char* argv[])
{
  char str[] = "Hello, world!\n";

  // Не используем printf() потому что это функция stdlib. 
  // Используем системный вызов write() в дескриптор 1 = STDOUT
  write(1, str, sizeof(str) - 1);
  
  _exit(0);
}