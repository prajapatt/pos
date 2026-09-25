bits 16
org 0x7c00

start:
	cli
	xor ax, ax
	mov ds, ax
	mov es, ax
	mov ss, ax
	mov sp, 0x7c00
	sti

	mov [boot_drive], dl
	mov si, loading_message
	call print

	mov dl, [boot_drive]
	mov ah, 0x41
	mov bx, 0x55aa
	int 0x13
	jc no_extensions
	cmp bx, 0xaa55
	jne no_extensions
	test cx, 1
	jz no_extensions

	mov word [disk_packet.sector_count], payload_sectors
	mov word [disk_packet.buffer_offset], stage2_load_offset
	mov word [disk_packet.buffer_segment], stage2_load_segment
	mov dl, [boot_drive]
	mov si, disk_packet
	mov ah, 0x42
	int 0x13
	jc disk_error

	jmp stage2_load_segment:stage2_load_offset

disk_error:
	mov si, error_message
	call print
	jmp halt

no_extensions:
	mov si, extensions_message
	call print
	jmp halt

halt:
	cli
	hlt
	jmp halt

print:
	lodsb
	test al, al
	jz .done
	mov ah, 0x0e
	mov bh, 0
	int 0x10
	jmp print
.done:
	ret

boot_drive db 0
loading_message db "CHUT-OS stage1: loading stage2", 13, 10, 0
error_message db "CHUT-OS stage1: disk read failed", 13, 10, 0
extensions_message db "CHUT-OS stage1: BIOS extensions unavailable", 13, 10, 0

disk_packet:
	db 0x10
	db 0
.sector_count dw 0
.buffer_offset dw 0
.buffer_segment dw 0
.start_lba dq 1

stage2_load_segment equ 0x0000
stage2_load_offset equ 0x8000
payload_sectors equ 16

times 510 - ($ - $$) db 0
dw 0xaa55
