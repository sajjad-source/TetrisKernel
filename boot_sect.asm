mov ah, 0x0e ; teletype BIOS routine

mov al, 'H' ; character to print
int 0x10 ; interrupt 0x10

jmp $ ; jump to current address (infinite loop)

times 510-($-$$) db 0 ; pad with zeros until 510 bytes (last 2 bytes are for 0xaa55)

dw 0xaa55 ; define word