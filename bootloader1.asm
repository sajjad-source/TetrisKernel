; bootloader1.asm - First stage bootloader

[ORG 0x7C00] ; BIOS loads the bootloader at 0x7C00 in memory

; Set up the stack
mov ax, 0x9000
mov ss, ax
mov sp, 0xFFFF

; Initialize the second stage bootloader location
mov ax, 0x1000
mov es, ax

; Read the second stage bootloader from the disk
xor ax, ax
mov ah, 0x02
mov al, 2 ; number of sectors to read
mov ch, 0
mov cl, 2
mov dh, 0
mov dl, 0x80
int 0x13

; Jump to the second stage bootloader
jmp 0x1000:0x0000

; BIOS Parameter Block (BPB) for a FAT12 filesystem
times 510 - ($-$$) db 0 ; Pad the remaining bytes with 0s
dw 0xAA55 ; Boot signature for BIOS
