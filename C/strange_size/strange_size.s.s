	.text
	.intel_syntax noprefix
	.file	"strange_size.c"
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
	sub	rsp, 32
	mov	dword ptr [rbp - 12], 0
	mov	dword ptr [rbp - 8], edi
	mov	qword ptr [rbp - 24], rsi
	mov	byte ptr [rbp - 2], 72
	mov	byte ptr [rbp - 1], 84
	movzx	eax, byte ptr [rbp - 2]
	movzx	ecx, byte ptr [rbp - 1]
	add	eax, ecx
	and	al, 127
	mov	byte ptr [rbp - 3], al
	movzx	esi, byte ptr [rbp - 2]
	movzx	edx, byte ptr [rbp - 1]
	movzx	ecx, byte ptr [rbp - 3]
	movabs	rdi, offset .L.str
	mov	al, 0
	call	printf
	xor	eax, eax
	add	rsp, 32
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
	.asciz	"%d + %d = %d\n"
	.size	.L.str, 14


	.ident	"clang version 7.0.0 (tags/RELEASE_700/final)"
	.section	".note.GNU-stack","",@progbits
