[org 0x7c00]
mov ah, 0x0e ; teletype BIOS routine

loop:
    mov al, [my_string + bx] ; get character from string
    cmp al, 0 ; check if null terminator
    je done ; if so, we're done
    int 0x10 ; print character
    inc bx ; increment index
    jmp loop ; repeat

done:
    jmp $ ; infinite loop

my_string:
    db 'Booting OS', 0 ; string with null terminator

times 510-($-$$) db 0 ; pad with zeros until 510 bytes (last 2 bytes are for 0xaa55)
dw 0xaa55 ; define word