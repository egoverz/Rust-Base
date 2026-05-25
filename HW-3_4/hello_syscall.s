.text
.globl main

main:
	pushq %rbp
	movq %rsp, %rbp

	movq $1, %rax # номер write syscall
	movq $1, %rdi # stdout
	movq $msg, %rsi # указатель на строку
	movq $14, %rdx # длина строки
	syscall 

	movq $60, %rax # номер exit syscall
	movq $0, %rdi 

	syscall 



.section .data
msg:
	.string "Hello, world!\n"