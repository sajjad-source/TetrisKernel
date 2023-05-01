[org 0x7c00] ; set origin to 0x7c00

mov bx, HELLO_MSG ; load address of HELLO_MSG into dx
call print_string ; call print_string function

call getline ; call getline function

mov bx, GOODBYE_MSG ; load address of GOODBYE_MSG into dx
call print_string ; call print_string function

int 0x18 ; call BIOS interrupt 0x18 (reboot)

jmp $ ; hang

%include "asm/print_string.asm" ; include print_string function
%include "asm/getline.asm" ; include getline function

; Data
HELLO_MSG:
    db 'Booting BoGoOS', 13, 10, 0 ; string + CR + LF + null terminator

GOODBYE_MSG:
    db 13, 10, 'Quitting', 0 ; string with null terminator

times 510-($-$$) db 0 ; pad with zeros until 510 bytes (last 2 bytes are for 0xaa55)
dw 0xaa55 ; define word