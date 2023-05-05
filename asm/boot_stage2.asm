[BITS 16]

EXTERN main

section .text.stage2
global stage2_main
stage2_main:
    ; Initialize registers and stack for Stage 2, if needed

    ; Call the Rust main function
    call main

    ; Hang the system
hang:
    hlt
    jmp hang
