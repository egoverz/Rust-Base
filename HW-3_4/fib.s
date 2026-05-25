.text
.globl main
.globl fibonacci

fibonacci:
	cmpq $0, %rdi # проверяем, что переданный аргумент не равен нулю
	je .equal # если равен, но прыгаем на equal и возвращаем 0

	movq $0, %rdx # начальное значение, которое использует для рассчёта числа
	movq $1, %rax  # начальное значение, которое использует для рассчёта числа

	movq $2, %rcx # этот регистр используем как счётчик, начинаем с 2, т.к. первые два числа уже посчитаны 0, 1

.loop_start:
	cmpq %rdi, %rcx # сравнием переданное число со счётчик
	jg .loop_end # цикл идёт пока счётчик не станет больше нашего числа
	movq %rax, %r8 # используем %r8 как промежуточное значение
	add %rdx, %rax # считаем следующее число фибоначчи 
	movq %r8, %rdx # сохраняем прожуточное число обратно в регистр, будем его использовать в следующих вычислениях

	incq %rcx # увеличиваем счётчик
	jmp .loop_start



.loop_end:
	ret

.equal:
	movq $0, %rax
	ret



main:
	pushq %rbp
	movq %rsp, %rbp

	movq $input, %rdi # указатель на форматную строку
	movq $num, %rsi # куда сохранить
	movq $0, %rax # целочисленное число
	call scanf
	movq num(%rip), %rdi # сохраняем переданное число в %rdi, т.к. это первый аргумент нашей функции fib, согласно abi

	call fibonacci # вызываем нашу функцию

	leaq msg(%rip), %rdi
	movq %rax, %rsi
	movb $0, %al
	call printf

	movq $0, %rax
	movq %rbp, %rsp
	pop %rbp
	ret


.section .data
msg:
	.string "Result: %d\n"
input:
	.string "%d"
num:
	.int 0