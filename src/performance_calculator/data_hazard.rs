use std::collections::HashMap;

use crate::{
    riscv_core::instruction::{Instruction, OpCodeType},
    utils::constants::NOP_INST,
};

#[derive(Debug, Clone)]
pub enum DataHazard {
    Raw(usize),
    War(usize),
    Waw(usize),
    None,
}

// Instrucoess de escrita (Possuem RD): U, J, I, L, R
// Instrucoes de leitura (Todos): I, L, B, S, R
// Instrucoess de leitura (Somente RS1): I, L
// Instrucoess de leitura (Possuem RS1 e RS2): B, S, R

// Start: Situacao 1 -> Only Nops
fn war_hazard(current_inst: Instruction, next_instructions: Vec<Instruction>) -> DataHazard {
    // Check for WAR hazards (Escrita-apos-Leitura)
    // ha um conflito WAR, onde uma instrucao (next_inst) tenta escrever em um registrador que esta sendo lido por uma instrucao anterior (current_inst).
    // Inst atual === leitura ----- Inst proxima(s) === escrita
    let mut amount_of_nops = 0;
    let mut nops_offset = 1;

    // Le o vetor de instrucoes de tras para frente
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
    // Check for WAW hazards (Escrita-apos-Escrita)
    // o conflito e no WAW, onde duas instrucoes (next_inst e current_inst) tentam escrever no mesmo registrador em uma ordem incorreta.
    // Inst atual === escrita ----- Inst proxima(s) === escrita
    let mut amount_of_nops = 0;
    let mut nops_offset = 1;

    // Le o vetor de instrucoes de tras para frente
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
                OpCodeType::I(_)
                | OpCodeType::L(_)
                | OpCodeType::J(_)
                | OpCodeType::R(_)
                | OpCodeType::U(_) => {
                    if current_inst.clone().get_rd() == next_inst.clone().get_rd() {
                        amount_of_nops += nops_offset;
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
    // Check for RAW hazards (Leitura-apos-Escrita)
    // ha um conflito RAW, onde uma instrucao (next_inst) tenta ler um registrador que foi escrito por uma instrucao anterior (current_inst).
    // Inst atual === escrita ----- Inst proxima(s) === leitura
    let mut amount_of_nops = 0;
    let mut nops_offset = 1;

    // Le o vetor de instrucoes de tras para frente
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
                        continue;
                    }
                }
                // Com RS1 e RS2
                OpCodeType::B(_) | OpCodeType::S(_) | OpCodeType::R(_) => {
                    if current_inst.clone().get_rd() == next_inst.clone().get_rs1()
                        || current_inst.clone().get_rd() == next_inst.clone().get_rs2()
                    {
                        amount_of_nops += nops_offset;
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
        let waw_hazard = waw_hazard(current_inst.clone(), next_two.clone());
        let raw_hazard = raw_hazard(current_inst.clone(), next_two.clone());

        inst_hazards.insert("WAR".to_string(), war_hazard.clone());
        inst_hazards.insert("WAW".to_string(), waw_hazard.clone());
        inst_hazards.insert("RAW".to_string(), raw_hazard.clone());

        hazards.push(inst_hazards);
    }

    hazards
}
// End: Situacao 1 -> Only Nops

// Start: Situacao 2 -> Forwarding
fn forwading_hazard(current_inst: Instruction, next_instructions: Vec<Instruction>) -> usize {
    for next in next_instructions.clone() {
        match current_inst.clone().get_opcode() {
            OpCodeType::L(_) => match next.clone().get_opcode() {
                // Somente RS1
                OpCodeType::I(_) | OpCodeType::L(_) => {
                    if current_inst.clone().get_rd() == next.clone().get_rs1() {
                        return 1;
                    }
                }
                // Com RS1 e RS2
                OpCodeType::B(_) | OpCodeType::S(_) | OpCodeType::R(_) => {
                    if current_inst.clone().get_rd() == next.clone().get_rs1()
                        || current_inst.clone().get_rd() == next.clone().get_rs2()
                    {
                        return 1;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
    0
}

pub fn check_for_hazards_with_forwarding(instructions: Vec<Instruction>) -> Vec<usize> {
    let mut hazards: Vec<usize> = Vec::new();

    for (index, current_inst) in instructions.iter().enumerate() {
        let mut next: Vec<Instruction> = Vec::new();

        for (index, next_inst) in instructions[index + 1..].iter().enumerate() {
            if index < 1 {
                next.push(next_inst.clone());
            }
        }

        let inst_hazard = forwading_hazard(current_inst.clone(), next.clone());
        hazards.push(inst_hazard);
    }

    hazards
}
// End: Situacao 2 -> Forwarding

// Start: Situacao 3 e Situacao 4 -> Reorder and Only Nops && Reorder and Forwarding with Nops
pub fn check_for_reorder(instructions: Vec<Instruction>) -> Vec<Option<Vec<bool>>> {
    // Retorna um matriz contendo bool se pode ou nao reordenar de acordo com a instrucao atual e as proximas duas
    let mut can_reorder: Vec<Option<Vec<bool>>> = Vec::new();
    let hazards = check_for_hazards(instructions.clone());

    for (index, current_inst) in instructions.iter().enumerate() {
        // Se a inst atual for nops, ignora
        if current_inst.clone().get_full_inst() == NOP_INST {
            can_reorder.push(None);
            continue;
        }

        // Se a inst atual for desvio, ignora
        match current_inst.clone().get_opcode() {
            OpCodeType::J(_) | OpCodeType::B(_) => {
                can_reorder.push(None);
                continue;
            }
            _ => (),
        }

        let mut next_two: Vec<Instruction> = Vec::new();

        let mut next_two_counter = index + 1;
        while next_two.len() < 2 && next_two_counter < instructions.len() {
            if instructions[next_two_counter].clone().get_full_inst() != NOP_INST {
                match instructions[next_two_counter].clone().get_opcode() {
                    OpCodeType::J(_) | OpCodeType::B(_) => {
                        // Nao faz nada
                        break;
                    }
                    _ => {
                        next_two.push(instructions[next_two_counter].clone());
                    }
                }
            }

            next_two_counter += 1;
        }

        let war = hazards[index].get("WAR").unwrap();
        let waw = hazards[index].get("WAW").unwrap();
        let raw = hazards[index].get("RAW").unwrap();

        let war_nops = match war {
            DataHazard::War(nops) => nops.clone(),
            _ => 0,
        };
        let waw_nops = match waw {
            DataHazard::Waw(nops) => nops.clone(),
            _ => 0,
        };
        let raw_nops = match raw {
            DataHazard::Raw(nops) => nops.clone(),
            _ => 0,
        };

        let mut can_reorder_first_inst = false;
        let mut can_reorder_second_inst = false;

        if war_nops == 2 || waw_nops == 2 || raw_nops == 2 {
            // Se as proximas inst tiver 2 nops de conflito, nao as mova
            can_reorder_first_inst = false;
            can_reorder_second_inst = false;
        } else if war_nops == 1 || waw_nops == 1 || raw_nops == 1 {
            // Se as proximas inst tiver 1 nops de conflito, mova a segunda
            can_reorder_first_inst = false; // Move 1 para cima
            can_reorder_second_inst = true; // Move 2 para cima
        }

        can_reorder.push(Some(vec![can_reorder_first_inst, can_reorder_second_inst]));
    }

    can_reorder
}
// End: Situacao 3 e Situacao 4 -> Reorder and Only Nops && Reorder and Forwarding with Nops
