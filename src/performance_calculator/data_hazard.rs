use crate::riscv_core::instruction::{Instruction, OpCodeType};

enum DataHazard {
    Raw(),
    War(),
    Waw(),
}

pub fn check_for_hazard(instructions: Vec<Instruction>, current_index: usize) -> bool {
    if instructions.len() < 2 {
        return false;
    }

    for (index, curent_inst) in instructions[2..current_index].iter().enumerate() {
        let prev_inst = instructions[index].clone();

        // Verifica se o RD eh o zero, se for nao precisa verificar os hazards
        if curent_inst.clone().get_rd() == "00000" {
            continue;
        }

        // Check for WAR hazards (Escrita-apos-Leitura) - NOK
        // ha um conflito WAR, onde uma instrucao tenta escrever em um registrador que esta sendo lido por uma instrucao posterior.
        // Inst atual === escrita ----- Inst anterior === leitura
        match prev_inst.clone().get_opcode() {
            // Somente RS1
            OpCodeType::I(_) | OpCodeType::L(_) => match curent_inst.clone().get_opcode() {
                OpCodeType::I(_)
                | OpCodeType::L(_)
                | OpCodeType::J(_)
                | OpCodeType::R(_)
                | OpCodeType::U(_) => {
                    if prev_inst.clone().get_rs1() == curent_inst.clone().get_rd() {
                        return true;
                    }
                }
                _ => (),
            },

            // Com RS1 e RS2
            OpCodeType::B(_) | OpCodeType::S(_) | OpCodeType::R(_) => {
                match curent_inst.clone().get_opcode() {
                    OpCodeType::I(_)
                    | OpCodeType::L(_)
                    | OpCodeType::J(_)
                    | OpCodeType::R(_)
                    | OpCodeType::U(_) => {
                        if prev_inst.clone().get_rs1() == curent_inst.clone().get_rd()
                            || prev_inst.clone().get_rs2() == curent_inst.clone().get_rd()
                        {
                            return true;
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        // Check for WAW hazards (Escrita-apos-Escrita) - NOK
        // o conflito e no WAW, onde duas instrucoes tentam escrever no mesmo registrador em uma ordem incorreta.
        // Inst atual === escrita ----- Inst anterior === escrita
        match prev_inst.clone().get_opcode() {
            OpCodeType::I(_)
            | OpCodeType::L(_)
            | OpCodeType::J(_)
            | OpCodeType::R(_)
            | OpCodeType::U(_)
            | OpCodeType::S(_) => match curent_inst.clone().get_opcode() {
                OpCodeType::I(_)
                | OpCodeType::L(_)
                | OpCodeType::J(_)
                | OpCodeType::R(_)
                | OpCodeType::U(_)
                | OpCodeType::S(_) => {
                    if prev_inst.clone().get_rd() == curent_inst.clone().get_rd() {
                        return true;
                    }
                }
                _ => (),
            },
            _ => (),
        }

        // Check for RAW hazards (Leitura-apos-Escrita) - NOK
        // ha um conflito RAW, onde uma instrucao tenta ler um registrador que foi escrito por uma instrucao anterior.
        // Inst atual === leitura ----- Inst anterior === escrita
        match prev_inst.clone().get_opcode() {
            OpCodeType::I(_)
            | OpCodeType::L(_)
            | OpCodeType::J(_)
            | OpCodeType::R(_)
            | OpCodeType::U(_) => match curent_inst.clone().get_opcode() {
                // Com RS1 e RS2
                OpCodeType::I(_)
                | OpCodeType::L(_)
                | OpCodeType::B(_)
                | OpCodeType::S(_)
                | OpCodeType::R(_) => {
                    if prev_inst.clone().get_rd() == curent_inst.clone().get_rs1()
                        || prev_inst.clone().get_rd() == curent_inst.clone().get_rs2()
                    {
                        // Insert NOP before the conflicted instruction
                        return true;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    false
}
