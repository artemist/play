	.text
	.intel_syntax noprefix
	.file	"sadd.c"
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
	mov	dword ptr [rbp - 20], 0
	mov	dword ptr [rbp - 16], edi
	mov	qword ptr [rbp - 32], rsi
	mov	dword ptr [rbp - 8], -2147483648
	mov	dword ptr [rbp - 4], -1073741824
	mov	edi, dword ptr [rbp - 8]
	mov	esi, dword ptr [rbp - 4]
	call	llvm.sadd.sat.i32
	mov	dword ptr [rbp - 12], eax
	mov	esi, dword ptr [rbp - 8]
	mov	edx, dword ptr [rbp - 4]
	mov	ecx, dword ptr [rbp - 12]
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
