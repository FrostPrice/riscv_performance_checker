use serde::{Deserialize, Serialize};

use crate::{
    config::db::Connection,
    models::bin_file::BinFile,
    riscv_core::{
        self,
        instruction::{Instruction, OpCodeType},
    },
    utils::constants::NOP_INST,
};

#[derive(Serialize, Deserialize)]
pub struct PerformanceCalculator {
    pub basic_information: BasicInformation,
}

#[derive(Serialize, Deserialize)]
pub struct BasicInformation {
    pub organization_clock_time: f32,
    pub bin_file_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct PerformanceCalculatorPipelineDTO {
    pub tclock: f32,
    pub bin_file_name: String,
}

impl PerformanceCalculator {
    pub fn calc_pipeline(
        performance_calculator_pipeline_dto: PerformanceCalculatorPipelineDTO,
        conn: &mut Connection,
    ) -> actix_web::Result<String, String> {
        // Start: Get info from database
        let bin_file = match BinFile::find_by_id(
            performance_calculator_pipeline_dto.bin_file_name.clone(),
            conn,
        ) {
            Ok(bin_file) => bin_file,
            Err(_) => {
                return Err(format!(
                    "Bin file {} not found",
                    performance_calculator_pipeline_dto.bin_file_name
                ))
            }
        };
        // End: Get info from database

        // 1. Insira o tempo de clock do Pipeline; - OK
        // 2. Escolha o arquivo com o programa em binário; - OK
        // 3. Execute todas as técnicas e calcule o desempenho de cada uma;
        // 4. Gere arquivos para cada solução; e
        // 5. Exiba todos os resultados.
        // Obs: Um bug comum na reordenação é tentar buscar uma instrução após a última
        // instrução.

        // Add instructions struct to Vector
        let mut instructions = Vec::<Instruction>::new();

        for line in bin_file.file.trim().lines() {
            let inst = riscv_core::instruction::Instruction::new(line);
            instructions.push(inst);
        }

        let only_nops = only_nops(&instructions);
        let mut only_nops_str = String::new();
        for i in only_nops {
            only_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        }

        let forwading_with_nops = forwading_with_nops(&instructions);
        let mut forwarding_with_nops_str = String::new();
        for i in forwading_with_nops {
            forwarding_with_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        }

        Ok(String::from("TODO"))
    }
}

fn only_nops(instructions: &Vec<Instruction>) -> Vec<Instruction> {
    // Considerar que não há nenhuma solução em hardware para conflitos e incluir NOPs, quando necessário, para evitar o conflito de dados.
    let mut nop_counter = 0;

    let mut instructions_with_nops = vec![instructions[0].clone()];

    for i in 1..instructions.clone().len() {
        let current = instructions[i].clone();
        let prev = instructions[i - 1].clone();

        instructions_with_nops.push(current.clone());

        // TODO: Validar se os formatos estão para o conflito correto e se o prev eh correto (ou se devo usar o current)
        // Instruções de escrita (Possuem RD): U, J, I, L, R
        // Instruções de leitura (Somente RS1): I, L
        // Instruções de leitura (Possuem RS1 e RS2): B, S, R
        // Provavelemente sera necessario comparar o formato da inst do prev com o formato da inst do current

        // Check for RAW hazards (Leitura-apos-Escrita)
        // ha um conflito RAW, onde uma instrucao tenta ler um registrador que foi escrito por uma instrucao anterior.
        match prev.clone().get_opcode() {
            OpCodeType::R(_)
            | OpCodeType::I(_)
            | OpCodeType::J(_)
            | OpCodeType::U(_)
            | OpCodeType::L(_)
            | OpCodeType::B(_)
            | OpCodeType::S(_) => {
                if current.clone().get_rs1() == prev.clone().get_rd()
                    || current.clone().get_rs2() == prev.clone().get_rd()
                {
                    // Insert NOP before the conflicted instruction
                    let nop = Instruction::new(NOP_INST);
                    instructions_with_nops.insert(i, nop);
                    nop_counter += 1;
                    println!("RAW Hazard");
                    continue;
                }
            }
        }

        // Check for WAR hazards (Escrita-apos-Leitura)
        // ha um conflito WAR, onde uma instrucao tenta escrever em um registrador que esta sendo lido por uma instrucao posterior.
        match prev.clone().get_opcode() {
            OpCodeType::R(_)
            | OpCodeType::I(_)
            | OpCodeType::J(_)
            | OpCodeType::U(_)
            | OpCodeType::L(_)
            | OpCodeType::B(_)
            | OpCodeType::S(_) => {
                if current.clone().get_rd() == prev.clone().get_rs1()
                    || current.clone().get_rd() == prev.clone().get_rs2()
                {
                    // Insert NOP before the conflicted instruction
                    let nop = Instruction::new(NOP_INST);
                    instructions_with_nops.insert(i, nop);
                    nop_counter += 1;
                    println!("WAR Hazard");
                    continue;
                }
            }
        }

        // Check for WAW hazards (Escrita-apos-Escrita)
        // o conflito e no WAW, onde duas instrucoes tentam escrever no mesmo registrador em uma ordem incorreta.
        match prev.clone().get_opcode() {
            OpCodeType::R(_)
            | OpCodeType::I(_)
            | OpCodeType::J(_)
            | OpCodeType::U(_)
            | OpCodeType::L(_)
            | OpCodeType::B(_)
            | OpCodeType::S(_) => {
                if prev.clone().get_rd() == current.clone().get_rd() {
                    // Insert NOP before the current instruction
                    let nop = Instruction::new(NOP_INST);
                    instructions_with_nops.insert(i, nop);
                    nop_counter += 1;
                    println!("WAW Hazard");
                    continue;
                }
            }
        }
    }

    println!("Only NOPs: {}", nop_counter);

    instructions_with_nops
}

fn forwading_with_nops(instructions: &Vec<Instruction>) -> Vec<Instruction> {
    // Considerar que foi implementada a técnica de forwarding e inserir NOPs, quando necessário, para evitar conflito de dados.

    // Somente inserir nops para instrucoes de formato L
    let mut nop_counter = 0;

    let mut forwading_with_nops = vec![instructions[0].clone()];

    for i in 1..instructions.clone().len() {
        let current = instructions[i].clone();
        let prev = instructions[i - 1].clone();

        forwading_with_nops.push(current.clone());

        match prev.clone().get_opcode() {
            OpCodeType::L(_) => {
                if prev.clone().get_rd() == current.clone().get_rd() {
                    // Insert NOP before the current instruction
                    let nop = Instruction::new(NOP_INST);
                    forwading_with_nops.insert(i, nop);
                    nop_counter += 1;
                    continue;
                }
            }
            _ => (),
        }
    }

    println!("Forwading with NOPs: {}", nop_counter);

    forwading_with_nops
}

fn rearange_with_only_nops() {
    // Considerar que não há nenhuma solução em hardware para conflitos e quando possível reordenar as instruções e quando não for possível inserir NOPs, para evitar conflito de dados.
    //     a. Por exemplo, é possível que o programa não tenha nenhuma instrução, a diante no código, para ser reordenada.
}

fn forwading_and_rearange_with_nops() {
    // Considerar que foi implementada a técnica de forwarding e quando possível reordenar as instruções e quando não for possível inserir NOPs, para evitar conflito de dados.
}

// NOP -> 00000000000000000000000000010011
