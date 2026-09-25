bits 16
org 0x8000

stage2_sectors equ 8

stage2_entry:
	cli
	xor ax, ax
	mov ds, ax
	mov es, ax
	mov ss, ax
	mov sp, 0x7c00
	sti

	call serial_init
	mov si, bios_message
	call bios_print
	mov si, serial_message
	call serial_print
	call check_long_mode
	jc unsupported_cpu

	cli
	call enable_a20
	lgdt [gdt_descriptor]
	mov eax, cr0
	or eax, 1
	mov cr0, eax
	jmp 0x08:protected_mode_entry

enable_a20:
	in al, 0x92
	or al, 2
	out 0x92, al
	ret

check_long_mode:
	pushfd
	pop eax
	mov ecx, eax
	xor eax, 0x00200000
	push eax
	popfd
	pushfd
	pop eax
	xor eax, ecx
	jz .unsupported
	mov eax, 0x80000000
	cpuid
	cmp eax, 0x80000001
	jb .unsupported
	mov eax, 0x80000001
	cpuid
	test edx, 0x20000000
	jz .unsupported
	clc
	ret
.unsupported:
	stc
	ret

unsupported_cpu:
	mov si, cpu_message
	call bios_print
	jmp idle

bits 32
protected_mode_entry:
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov ss, ax
	mov esp, 0x7000

	call setup_paging
	mov eax, cr4
	or eax, 0x20
	mov cr4, eax

	mov ecx, 0xc0000080
	rdmsr
	or eax, 0x100
	wrmsr

	mov eax, cr0
	or eax, 0x80000000
	mov cr0, eax
	jmp 0x18:long_mode_entry

setup_paging:
	mov edi, 0x1000
	xor eax, eax
	mov ecx, 3072
	rep stosd

	mov dword [0x1000], 0x2003
	mov dword [0x2000], 0x3003
	mov dword [0x3000], 0x0083
	mov eax, 0x1000
	mov cr3, eax
	ret

bits 64
long_mode_entry:
	mov ax, 0x20
	mov ds, ax
	mov es, ax
	mov ss, ax
	mov rsp, 0x7000
	mov rsi, long_mode_message
	call serial64_print
	call 0x9000

long_mode_idle:
	hlt
	jmp long_mode_idle

serial64_print:
	lodsb
	test al, al
	jz .done
	call serial64_write
	jmp serial64_print
.done:
	ret

serial64_write:
	push rax
	push rdx
.wait:
	mov dx, 0x3fd
	in al, dx
	test al, 0x20
	jz .wait
	pop rdx
	pop rax
	mov dx, 0x3f8
	out dx, al
	ret

bits 16

gdt_start:
	dq 0x0000000000000000
	dq 0x00cf9a000000ffff
	dq 0x00cf92000000ffff
	dq 0x00af9a000000ffff
	dq 0x00cf92000000ffff
gdt_end:
gdt_descriptor:
	dw gdt_end - gdt_start - 1
	dd gdt_start

idle:
	hlt
	jmp idle

serial_init:
	mov dx, 0x3f9
	xor al, al
	out dx, al
	mov dx, 0x3fb
	mov al, 0x80
	out dx, al
	mov dx, 0x3f8
	mov al, 1
	out dx, al
	inc dx
	xor al, al
	out dx, al
	mov dx, 0x3fb
	mov al, 3
	out dx, al
	mov dx, 0x3fa
	mov al, 0xc7
	out dx, al
	mov dx, 0x3fc
	mov al, 0x0b
	out dx, al
	ret

bios_print:
	lodsb
	test al, al
	jz .done
	mov ah, 0x0e
	mov bh, 0
	int 0x10
	jmp bios_print
.done:
	ret

serial_print:
	lodsb
	test al, al
	jz .done
	call serial_write
	jmp serial_print
.done:
	ret

serial_write:
	push ax
	push dx
	mov ah, al
.wait:
	mov dx, 0x3fd
	in al, dx
	test al, 0x20
	jz .wait
	mov al, ah
	mov dx, 0x3f8
	out dx, al
	pop dx
	pop ax
	ret

bios_message db "CHUT-OS stage2: kernel entry reached", 13, 10, 0
serial_message db "CHUT-OS stage2: serial online", 13, 10, 0
long_mode_message db "CHUT-OS: x86_64 kernel entry reached", 13, 10, 0
cpu_message db "CHUT-OS stage2: x86_64 long mode unavailable", 13, 10, 0

times stage2_sectors * 512 - ($ - $$) db 0
