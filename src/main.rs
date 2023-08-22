use std::{collections::HashMap, env, fs, io};

use crate::riscv_core::instruction::OpCodeType;

mod riscv_core;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Could not read file!");

    // Get information about clock
    println!("/////////////////////////////////////////////////////////////");
    println!("Por favor, informe o tempo de clock: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let clock = input.trim().parse::<f32>().unwrap();

    // START: Read Informations about organization A
    let mut instructions_cpi_org_a: HashMap<OpCodeType, f32> = HashMap::new();

    println!("/////////////////////////////////////////////////////////////");
    println!("Por favor, informe o CPI das instrucoes da organizacao A:");

    input = String::new();
    println!("CPI de instrucoes do tipo R: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_a.insert(
        OpCodeType::R("R".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo I: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_a.insert(
        OpCodeType::I("I".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo L: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_a.insert(
        OpCodeType::L("L".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo S: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_a.insert(
        OpCodeType::S("S".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo B: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_a.insert(
        OpCodeType::B("B".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo U: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_a.insert(
        OpCodeType::U("U".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo J: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_a.insert(
        OpCodeType::J("J".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    // END: Read Informations about organization A

    // START: Read Informations about organization B
    let mut instructions_cpi_org_b: HashMap<OpCodeType, f32> = HashMap::new();

    println!("/////////////////////////////////////////////////////////////");
    println!("Por favor, informe o CPI das instrucoes da organizacao B:");

    println!("CPI de instrucoes do tipo R: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_b.insert(
        OpCodeType::R("R".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo I: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_b.insert(
        OpCodeType::I("I".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo L: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_b.insert(
        OpCodeType::L("L".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo S: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_b.insert(
        OpCodeType::S("S".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo B: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_b.insert(
        OpCodeType::B("B".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo U: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_b.insert(
        OpCodeType::U("U".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );

    input = String::new();
    println!("CPI de instrucoes do tipo J: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    instructions_cpi_org_b.insert(
        OpCodeType::J("J".to_string()),
        input.trim().parse::<f32>().unwrap(),
    );
    // END: Read Informations about organization B

    let total_instrucoes = contents.lines().count();
    let mut total_ciclos_a: f32 = 0.0;
    let mut total_ciclos_b: f32 = 0.0;

    for line in contents.trim().lines() {
        let inst = riscv_core::instruction::Instruction::new(line);

        let opcode = inst.get_opcode();

        let cpi_a: &f32;
        let cpi_b: &f32;

        match opcode {
            OpCodeType::R(_) => {
                cpi_a = match instructions_cpi_org_a.get(&OpCodeType::R("R".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
                cpi_b = match instructions_cpi_org_b.get(&OpCodeType::R("R".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
            }
            OpCodeType::I(_) => {
                cpi_a = match instructions_cpi_org_a.get(&OpCodeType::I("I".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
                cpi_b = match instructions_cpi_org_b.get(&OpCodeType::I("I".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
            }
            OpCodeType::L(_) => {
                cpi_a = match instructions_cpi_org_a.get(&OpCodeType::L("L".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
                cpi_b = match instructions_cpi_org_b.get(&OpCodeType::L("L".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
            }
            OpCodeType::S(_) => {
                cpi_a = match instructions_cpi_org_a.get(&OpCodeType::S("S".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
                cpi_b = match instructions_cpi_org_b.get(&OpCodeType::S("S".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
            }
            OpCodeType::B(_) => {
                cpi_a = match instructions_cpi_org_a.get(&OpCodeType::B("B".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
                cpi_b = match instructions_cpi_org_b.get(&OpCodeType::B("B".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
            }
            OpCodeType::U(_) => {
                cpi_a = match instructions_cpi_org_a.get(&OpCodeType::U("U".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
                cpi_b = match instructions_cpi_org_b.get(&OpCodeType::U("U".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
            }
            OpCodeType::J(_) => {
                cpi_a = match instructions_cpi_org_a.get(&OpCodeType::J("J".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
                cpi_b = match instructions_cpi_org_b.get(&OpCodeType::J("J".to_string())) {
                    Some(cpi) => cpi,
                    None => &0.0,
                };
            }
        }

        total_ciclos_a = total_ciclos_a + cpi_a;
        total_ciclos_b = total_ciclos_b + cpi_b;
    }

    let tempo_execucao_a = total_ciclos_a as f32 * clock;
    let tempo_execucao_b = total_ciclos_b as f32 * clock;

    // Os ciclos por instrução (CPI) médio para o programa em cada organização;
    println!("/////////////////////////////////////////////////////////////");
    println!(
        "CPI medio para o programa na organizacao A: {}",
        total_ciclos_a / total_instrucoes as f32
    );
    println!(
        "CPI medio para o programa na organizacao B: {}",
        total_ciclos_b / total_instrucoes as f32
    );

    // O tempo de execução para o programa em cada organização;
    println!("/////////////////////////////////////////////////////////////");
    println!(
        "Tempo de execucao para o programa na organizacao A: {}",
        tempo_execucao_a
    );
    println!(
        "Tempo de execucao para o programa na organizacao B: {}",
        tempo_execucao_b
    );

    // A organização mais rápida (ou mais lenta), e o quão mais rápida (ou lenta) ela é;
    println!("/////////////////////////////////////////////////////////////");
    if tempo_execucao_a < tempo_execucao_b {
        println!("A organizacao A e mais rapida que a organizacao B");
        println!(
            "A organizacao A e {} vezes mais rapida que a organizacao B",
            tempo_execucao_b / tempo_execucao_a
        );
    } else if tempo_execucao_a > tempo_execucao_b {
        println!("A organizacao B e mais rapida que a organizacao A");
        println!(
            "A organizacao B e {} vezes mais rapida que a organizacao A",
            tempo_execucao_a / tempo_execucao_b
        );
    } else {
        println!("As organizacoes A e B sao igualmente rapidas");
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
