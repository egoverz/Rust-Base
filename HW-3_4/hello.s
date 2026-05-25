.text
.globl main

main:
	pushq %rbp
	movq %rsp, %rbp

	leaq msg(%rip), %rdi # загружаем нашу строку из .rodata
	call puts # используем puts для вывода

	movq $0, %rax # возвращаемое значение из main

	leave
	retq


.section .data
msg:
	.string "Hello, world!"