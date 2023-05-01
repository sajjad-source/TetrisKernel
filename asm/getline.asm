getline:
    pusha
    gloop:
        mov ah, 0x00 ; set up for keyboard read
        int 0x16 ; blocking read from keyboard
        cmp al, 0x0d ; check for enter key
        je gend ; if enter key, end
        mov ah, 0x0e
        int 0x10 ; echo character
        jmp gloop ; loop back
    gend:
        popa
        ret