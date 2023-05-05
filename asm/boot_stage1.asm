[BITS 16]
[org 0x7C00]

section .multiboot
dd 0x1BADB002 ; magic
dd 0x00        ; flags
dd - (0x1BADB002 + 0x00) ; checksum

section .text
global _start
_start:
    ; Set up disk read parameters
    mov ah, 0x02         ; BIOS read sectors function (0x02)
    mov al, [num_sectors] ; Number of sectors to read
    mov ch, 0            ; Cylinder number (0)
    mov cl, 2            ; Sector number (2, since LBA = 1)
    mov dh, 0            ; Head number (0)
    mov dl, 0x80         ; Drive number (0x80 for the first hard disk)
    mov bx, 0x8000       ; Buffer address to read the sector into (0x8000)
    mov es, bx
    xor bx, bx

    ; Call BIOS interrupt 0x13 to read the sector(s)
    int 0x13

    ; Check for errors
    jc disk_error

    ; Jump to the memory location where Stage 2 is loaded (0x8000)
    jmp 0x8000

disk_error:
    ; Handle disk read errors (e.g., print an error message and halt)
    mov ah, 0x0e ; teletype BIOS routine
    mov bx, ERROR_MSG ; load string address into bx
    loop:
        mov al, [bx] ; get character from string
        cmp al, 0 ; check if null terminator
        je end ; go to end
        int 0x10 ; print character
        inc bx ; increment index
        jmp loop ; repeat
    end:
        hlt
    

section .data
num_sectors db 1
ERROR_MSG:
    db "Disk read error!", 0