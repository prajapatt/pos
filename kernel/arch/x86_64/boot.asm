bits 64
section .text
    global start
    extern kernel_entry

start:
    cli
    mov rsp, stack_top
    call kernel_entry

hang:
    cli
    hlt
    jmp hang

section .bss
align 16
stack_bottom:
    resb 16384
stack_top:
