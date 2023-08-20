fn main() {}

// ################################
// .text

// 	addi s0, zero, 5
// 	addi s1, zero, 10

// for:
// 	beq  t0, s0, fim
// 	add  s1, s1, t0
// 	addi t0, t0, 1
// 	jal  zero, for
// fim:
// ################################

///// TODO:
/*
Solicitar input de informações de Ciclo por instrução de 2 organizações diferente
Solicitar input de informação de clock (Tempo de relógio)
Calcular o desempenho de ambas as organizações
Informar qual a organização mais rápida (ou mais lenta), e o quao mais rapida (ou lenta) ela é
Contar quantidade de instruções executadas
O teste ira usar o monociclo e multiciclo
*/

/////////////////////////////////////////
/*
Importante:
Cada formato de insrução terá um CPI:
R(String),
I(String),
S(String),
B(String),
U(String),
J(String),
L(String),
*/
