[org 0x7c00] ; set origin to 0x7c00
mov ah, 0x0e ; teletype BIOS routine

call print_string ; print string
jmp $

print_string:
    loop:
        mov al, [my_string + bx] ; get character from string
        cmp al, 0 ; check if null terminator
        je end ; go to end
        int 0x10 ; print character
        inc bx ; increment index
        jmp loop ; repeat
    end:
        ret

my_string:
    db 'Booting OS', 0 ; string with null terminator

times 510-($-$$) db 0 ; pad with zeros until 510 bytes (last 2 bytes are for 0xaa55)
dw 0xaa55 ; define word