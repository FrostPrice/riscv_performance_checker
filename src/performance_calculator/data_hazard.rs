use std::collections::HashMap;

use crate::riscv_core::instruction::{Instruction, OpCodeType};

#[derive(Debug, Clone)]
pub enum DataHazard {
    Raw(usize),
    War(usize),
    Waw(usize),
    None,
}

// Instruções de escrita (Possuem RD): U, J, I, L, R
// Instruções de leitura (Somente RS1): I, L
// Instruções de leitura (Possuem RS1 e RS2): B, S, R

fn war_hazard(current_inst: Instruction, next_instructions: Vec<Instruction>) -> DataHazard {
    // Check for WAR hazards (Escrita-apos-Leitura) - NOK
    // ha um conflito WAR, onde uma instrucao (next_inst) tenta escrever em um registrador que esta sendo lido por uma instrucao anterior (current_inst).
    // Inst atual === leitura ----- Inst proxima(s) === escrita

    let mut amount_of_nops = 0;
    let mut nops_offset = 1;

    for (index, next_inst) in next_instructions.iter().rev().enumerate() {
        // Quanto mais proximo da instrucao atual, maior o numero de nops
        if amount_of_nops == 0 && index != 0 || next_instructions.len() == 1 {
            nops_offset = 2
        };

        match current_inst.clone().get_opcode() {
            // Somente RS1
            OpCodeType::I(_) | OpCodeType::L(_) => match next_inst.clone().get_opcode() {
                OpCodeType::U(_)
                | OpCodeType::J(_)
                | OpCodeType::I(_)
                | OpCodeType::L(_)
                | OpCodeType::R(_) => {
                    if current_inst.clone().get_rs1() == next_inst.clone().get_rd() {
                        amount_of_nops += nops_offset;
                        println!("WAR Hazard");
                        continue;
                    }
                }
                _ => (), // Nao faz nada
            },
            // Com RS1 e RS2
            OpCodeType::B(_) | OpCodeType::S(_) | OpCodeType::R(_) => {
                match next_inst.clone().get_opcode() {
                    OpCodeType::U(_)
                    | OpCodeType::J(_)
                    | OpCodeType::I(_)
                    | OpCodeType::L(_)
                    | OpCodeType::R(_) => {
                        if current_inst.clone().get_rs1() == next_inst.clone().get_rd()
                            || current_inst.clone().clone().get_rs2() == next_inst.clone().get_rd()
                        {
                            amount_of_nops += nops_offset;
                            println!("WAR Hazard");
                            continue;
                        }
                    }
                    _ => (), // Nao faz nada
                }
            }
            _ => (),
        }
    }

    if amount_of_nops == 0 {
        return DataHazard::None;
    }

    DataHazard::War(amount_of_nops)
}

fn waw_hazard(current_inst: Instruction, next_instructions: Vec<Instruction>) -> DataHazard {
    // Check for WAW hazards (Escrita-apos-Escrita) - NOK
    // o conflito e no WAW, onde duas instrucoes (next_inst e current_inst) tentam escrever no mesmo registrador em uma ordem incorreta.
    // Inst atual === escrita ----- Inst proxima(s) === escrita
    let mut amount_of_nops = 0;
    let mut nops_offset = 1;

    for (index, next_inst) in next_instructions.iter().rev().enumerate() {
        // Quanto mais proximo da instrucao atual, maior o numero de nops
        if amount_of_nops == 0 && index != 0 || next_instructions.len() == 1 {
            nops_offset = 2
        };

        match current_inst.clone().get_opcode() {
            OpCodeType::I(_)
            | OpCodeType::L(_)
            | OpCodeType::J(_)
            | OpCodeType::R(_)
            | OpCodeType::U(_)
            | OpCodeType::S(_) => match next_inst.clone().get_opcode() {
                OpCodeType::I(_)
                | OpCodeType::L(_)
                | OpCodeType::J(_)
                | OpCodeType::R(_)
                | OpCodeType::U(_)
                | OpCodeType::S(_) => {
                    if current_inst.clone().get_rd() == next_inst.clone().get_rd() {
                        amount_of_nops += nops_offset;
                        println!("WAW Hazard");
                        continue;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    if amount_of_nops == 0 {
        return DataHazard::None;
    }

    DataHazard::Waw(amount_of_nops)
}

fn raw_hazard(current_inst: Instruction, next_instructions: Vec<Instruction>) -> DataHazard {
    // Check for RAW hazards (Leitura-apos-Escrita) - OK
    // ha um conflito RAW, onde uma instrucao (next_inst) tenta ler um registrador que foi escrito por uma instrucao anterior (current_inst).
    // Inst atual === escrita ----- Inst proxima(s) === leitura
    let mut amount_of_nops = 0;
    let mut nops_offset = 1;

    for (index, next_inst) in next_instructions.iter().rev().enumerate() {
        // Quanto mais proximo da instrucao atual, maior o numero de nops
        if amount_of_nops == 0 && index != 0 || next_instructions.len() == 1 {
            nops_offset = 2
        };

        match current_inst.clone().get_opcode() {
            OpCodeType::I(_)
            | OpCodeType::L(_)
            | OpCodeType::J(_)
            | OpCodeType::R(_)
            | OpCodeType::U(_) => match next_inst.clone().get_opcode() {
                // Somente RS1
                OpCodeType::I(_) | OpCodeType::L(_) => {
                    if current_inst.clone().get_rd() == next_inst.clone().get_rs1() {
                        amount_of_nops += nops_offset;
                        println!("RAW Hazard");
                        continue;
                    }
                }
                // Com RS1 e RS2
                OpCodeType::B(_) | OpCodeType::S(_) | OpCodeType::R(_) => {
                    if current_inst.clone().get_rd() == next_inst.clone().get_rs1()
                        || current_inst.clone().get_rd() == next_inst.clone().get_rs2()
                    {
                        amount_of_nops += nops_offset;
                        println!("RAW Hazard");
                        continue;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    if amount_of_nops == 0 {
        return DataHazard::None;
    }

    DataHazard::Raw(amount_of_nops)
}

pub fn check_for_hazards(instructions: Vec<Instruction>) -> Vec<HashMap<String, DataHazard>> {
    let mut hazards: Vec<HashMap<String, DataHazard>> = Vec::new();

    for (index, current_inst) in instructions.iter().enumerate() {
        let mut next_two: Vec<Instruction> = Vec::new();
        let mut inst_hazards: HashMap<String, DataHazard> = HashMap::new();

        for (index, next_inst) in instructions[index + 1..].iter().enumerate() {
            if index < 2 {
                next_two.push(next_inst.clone());
            }
        }

        let war_hazard = war_hazard(current_inst.clone(), next_two.clone());
        inst_hazards.insert("WAR".to_string(), war_hazard.clone());

        match war_hazard {
            DataHazard::War(_) => {
                inst_hazards.insert("WAW".to_string(), DataHazard::None);
                inst_hazards.insert("RAW".to_string(), DataHazard::None);
                hazards.push(inst_hazards);
                continue;
            }
            _ => (),
        }

        let waw_hazard = waw_hazard(current_inst.clone(), next_two.clone());
        inst_hazards.insert("WAW".to_string(), waw_hazard.clone());

        match waw_hazard {
            DataHazard::Waw(_) => {
                inst_hazards.insert("RAW".to_string(), DataHazard::None);
                hazards.push(inst_hazards);
                continue;
            }
            _ => (),
        }

        let raw_hazard = raw_hazard(current_inst.clone(), next_two.clone());
        inst_hazards.insert("RAW".to_string(), raw_hazard.clone());

        hazards.push(inst_hazards);
    }

    println!("Hazards: {:?}", hazards);

    hazards
}
