print_string:
    pusha ; save registers
    mov ah, 0x0e ; teletype BIOS routine
    loop:
        mov al, [bx] ; get character from string
        cmp al, 0 ; check if null terminator
        je end ; go to end
        int 0x10 ; print character
        inc bx ; increment index
        jmp loop ; repeat
    end:
        popa ; restore registers
        ret