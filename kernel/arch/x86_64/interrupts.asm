bits 64
section .text
    global isr0, isr1, isr2, isr3, isr4, isr5, isr6, isr7, isr8, isr9
    global isr10, isr11, isr12, isr13, isr14, isr15, isr16, isr17, isr18, isr19
    global isr20, isr21, isr22, isr23, isr24, isr25, isr26, isr27, isr28, isr29
    global isr30, isr31
    extern isr_handler

%macro ISR_NOERROR 1
isr%1:
    cli
    push qword 0
    push qword %1
    jmp common_interrupt
%endmacro

%macro ISR_ERROR 1
isr%1:
    cli
    push qword %1
    jmp common_interrupt
%endmacro

ISR_NOERROR 0
ISR_NOERROR 1
ISR_NOERROR 2
ISR_NOERROR 3
ISR_NOERROR 4
ISR_NOERROR 5
ISR_NOERROR 6
ISR_NOERROR 7
ISR_ERROR   8
ISR_NOERROR 9
ISR_ERROR  10
ISR_ERROR  11
ISR_ERROR  12
ISR_ERROR  13
ISR_ERROR  14
ISR_NOERROR 15
ISR_NOERROR 16
ISR_NOERROR 17
ISR_NOERROR 18
ISR_NOERROR 19
ISR_NOERROR 20
ISR_NOERROR 21
ISR_NOERROR 22
ISR_NOERROR 23
ISR_NOERROR 24
ISR_NOERROR 25
ISR_NOERROR 26
ISR_NOERROR 27
ISR_NOERROR 28
ISR_NOERROR 29
ISR_NOERROR 30
ISR_NOERROR 31

common_interrupt:
    push rbp
    mov rbp, rsp
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    mov rdi, [rbp + 16]
    call isr_handler

    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    pop rbp
    add rsp, 16
    sti
    iretq
