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
	sub	rsp, 80
	mov	dword ptr [rbp - 56], 0
	mov	dword ptr [rbp - 52], edi
	mov	qword ptr [rbp - 64], rsi
	mov	byte ptr [rbp - 24], 1
	mov	qword ptr [rbp - 32], -1
	mov	byte ptr [rbp - 8], 1
	mov	qword ptr [rbp - 16], -1
	movzx	eax, byte ptr [rbp - 24]
	mov	rcx, qword ptr [rbp - 32]
	movzx	edx, byte ptr [rbp - 8]
	mov	rsi, qword ptr [rbp - 16]
	add	rcx, rsi
	adc	rax, rdx
	mov	qword ptr [rbp - 48], rcx
	and	eax, 1
	mov	byte ptr [rbp - 40], al
	movzx	edx, byte ptr [rbp - 24]
	mov	rsi, qword ptr [rbp - 32]
	movzx	r8d, byte ptr [rbp - 8]
	mov	rcx, qword ptr [rbp - 16]
	movzx	eax, byte ptr [rbp - 40]
	mov	r9, qword ptr [rbp - 48]
	mov	qword ptr [rsp], rax
	mov	edi, offset .L.str
	xor	eax, eax
	call	printf
	xor	eax, eax
	add	rsp, 80
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
	.asciz	"%lu + %lu = %lu\n"
	.size	.L.str, 17


	.ident	"clang version 7.0.0 (tags/RELEASE_700/final)"
	.section	".note.GNU-stack","",@progbits
