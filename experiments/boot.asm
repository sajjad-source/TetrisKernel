section .text
    global _start

_start:
    ; Set the VGA text mode explicitly
    mov ah, 0x00   ; Function 0x00 - Set Video Mode
    mov al, 0x03   ; Mode 0x03 - 80x25 text mode
    int 0x10       ; Call video interrupt

    ; Load the kernel
    mov ah, 0x02  ; BIOS read sectors function
    mov al, 1     ; number of sectors to read
    mov ch, 0     ; cylinder number
    mov dh, 0     ; head number
    mov cl, 2     ; sector number
    mov bx, 0x8000; buffer to read to
    int 0x13      ; BIOS interrupt

    jmp 0x8000    ; jump to the loaded kernel

times 510 - ($ - $$) db 0   ; Pad the rest of the file with 0s up to 510 bytes
dw 0xAA55                   ; Boot signature
