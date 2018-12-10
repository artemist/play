	.text
	.intel_syntax noprefix
	.file	"i65.c"
	.globl	main                    # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:
	push	rbp
	.cfi_def_cfa_offset 16
	.cfi_offset rbp, -16
	mov	rbp, rsp
	.cfi_def_cfa_register rbp
	sub	rsp, 64
	mov	dword ptr [rbp - 40], 0
	mov	dword ptr [rbp - 36], edi
	mov	qword ptr [rbp - 64], rsi
	movabs	rax, -9223372036854775808
	mov	qword ptr [rbp - 32], rax
	mov	byte ptr [rbp - 24], 1
	mov	qword ptr [rbp - 16], rax
	mov	byte ptr [rbp - 8], 1
	movzx	eax, byte ptr [rbp - 24]
	mov	rcx, qword ptr [rbp - 32]
	movzx	edx, byte ptr [rbp - 8]
	mov	rsi, qword ptr [rbp - 16]
	add	rcx, rsi
	adc	rax, rdx
	shld	rax, rcx, 63
	mov	qword ptr [rbp - 56], rax
	mov	byte ptr [rbp - 48], 0
	mov	rsi, qword ptr [rbp - 32]
	mov	rdx, qword ptr [rbp - 16]
	mov	rcx, qword ptr [rbp - 56]
	movabs	rdi, offset .L.str
	mov	al, 0
	call	printf
	xor	eax, eax
	add	rsp, 64
	pop	rbp
	.cfi_def_cfa rsp, 8
	ret
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.type	.L.str,@object          # @.str
	.section	.rodata.str1.1,"aMS",@progbits,1
.L.str:
	.asciz	"(%lu + %lu) >> 1 = %lu\n"
	.size	.L.str, 24


	.ident	"clang version 7.0.0 (tags/RELEASE_700/final)"
	.section	".note.GNU-stack","",@progbits
