[org 0x7c00] ; set origin to 0x7c00

mov bx, HELLO_MSG ; load address of HELLO_MSG into dx
call print_string ; call print_string function

mov bx, GOODBYE_MSG ; load address of GOODBYE_MSG into dx
call print_string ; call print_string function

jmp $ ; hang

%include "print_string.asm" ; include print_string function

; Data
HELLO_MSG:
    db 'Booting OS', 13, 10, 0 ; string + CR + LF + null terminator

GOODBYE_MSG:
    db 'Quitting', 0 ; string with null terminator

times 510-($-$$) db 0 ; pad with zeros until 510 bytes (last 2 bytes are for 0xaa55)
dw 0xaa55 ; define word