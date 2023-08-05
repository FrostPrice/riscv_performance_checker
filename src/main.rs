use std::env;
use std::fs;

// RISCV Instruction type op code:
// R -> 0110011 === CPI = 1
// I -> 1110011 0010011 0001111 1100111 0000011 === CPI = 2
// S -> 0100011 === CPI = 3
// B -> 1100011 === CPI = 4
// U -> 0110111 0010111 === CPI = 5
// J -> 1101111 === CPI = 6

// Cria "Classe" para a instrução geral e depois criar sub classes para cada tipo de formato

// Pegar informação da CPU do computador
// Talvez usar sqlite

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Could not read file!");

    for line in contents.trim().lines() {
        let last_seven = &line[line.len() - 7..];
        println!("{}", last_seven);
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
