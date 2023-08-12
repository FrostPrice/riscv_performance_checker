use std::env;
use std::fs;

mod riscv_core;

// Pegar informação da CPU do computador
// Usar sqlite

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];
    // println!("Reading file {}", file_path);

    // let contents = fs::read_to_string(file_path).expect("Could not read file!");
    let contents = fs::read_to_string("./riscv_bin_dump/laco5x.txt").expect("Could not read file!");

    for line in contents.trim().lines() {
        let inst = riscv_core::instruction::Instruction::new(line);

        let opcode = inst.clone().get_opcode();
        let funct3 = inst.get_func3();
        println!("OpCode: {:?}", opcode);
        println!("Funct3: {:?}", funct3);
    }
}

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
// Solicitar input de informações de Ciclo por instrução de 2 organizações diferente
// Calcular o desempenho de ambas as organizações
// Informar qual a organização mais rápida (ou mais lenta), e o quao mais rapida (ou lenta) ela é
// Preparar código para implementar uma arquitetura Pipeline
// Contar quantidade de instruções executadas
