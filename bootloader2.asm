; bootloader2.asm - Second stage bootloader

[ORG 0x0000]

mov ah, 0x0e
mov al, '1'
int 0x10

; Fill the rest of the segment with zeros
times 512 - ($-$$) db 0

mov ah, 0x0e
mov al, '2'
int 0x10

mov ah, 0x0e
mov al, '3'
int 0x10

mov ah, 0x0e
mov al, '4'
int 0x10

mov ah, 0x0e
mov al, '5'
int 0x10
jmp $