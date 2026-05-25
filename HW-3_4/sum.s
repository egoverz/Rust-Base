.text
.globl main
.globl add


add:
	addq %rsi, %rdi # %rdi = %rdi + %rsi 
	movq %rdi, %rax # возвращаемое значение должно быть в %rax согласно abi
	ret


main:
	pushq %rbp
	movq %rsp, %rbp

	movq $15, %rdi # первый аргумент, используем %rdi согласно abi
	movq $10, %rsi # второй аргумент, используем %rsi согласно abi

	call add

	leaq msg(%rip), %rdi # форматная строка для printf
	movq %rax, %rsi # первый аргумент для форматной строки
	movb $0, %al
	call printf # вызываем printf

	movq $0, %rax # возвращаемое значение из main
	leave
	ret


.section .data
msg:
	.string "Result: %d\n"