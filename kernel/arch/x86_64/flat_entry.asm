bits 64
org 0x9000

flat_kernel_entry:
	mov rsi, kernel_message

serial_write:
	lodsb
	test al, al
	jz kernel_halt
	push rax
	push rdx
.wait:
	mov dx, 0x3fd
	in al, dx
	test al, 0x20
	jz .wait
	mov dx, 0x3f8
	pop rdx
	pop rax
	out dx, al
	jmp serial_write

kernel_halt:
	cli
	hlt
	jmp kernel_halt

kernel_message db "CHUT-OS: freestanding x86_64 kernel payload online", 13, 10, 0

times 4096 - ($ - $$) db 0
