#include "getch.h"

static inline char
getch (void)
{
   uint16_t inchar;

   __asm__ __volatile__ ("int $0x16\n\t"
            : "=a"(inchar)
            : "0"(0x0));

   return ((char)inchar);
}
