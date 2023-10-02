use std::fs;

use serde::{Deserialize, Serialize};

use crate::{
    config::db::Connection,
    models::bin_file::BinFile,
    performance_calculator::data_hazard::check_for_hazards,
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

        // Start: Implement techniques
        // Add instructions struct to Vector
        let mut instructions = Vec::<Instruction>::new();

        for line in bin_file.file.trim().lines() {
            let inst = riscv_core::instruction::Instruction::new(line);
            instructions.push(inst);
        }

        let only_nops = only_nops_test(instructions.clone());
        let mut only_nops_str = String::new();
        for i in only_nops {
            only_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        }

        // let forwading_with_nops = forwading_with_nops(instructions.clone());
        // let mut forwarding_with_nops_str = String::new();
        // for i in forwading_with_nops {
        //     forwarding_with_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        // }

        // let reorder_with_only_nops = reorder_with_only_nops(instructions.clone());
        // let mut reorder_with_only_nops_str = String::new();
        // for i in reorder_with_only_nops {
        //     reorder_with_only_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        // }

        // let forwading_and_reorder_with_nops = forwading_and_reorder_with_nops(instructions.clone());
        // let mut forwading_and_reorder_with_nops_str = String::new();
        // for i in forwading_and_reorder_with_nops {
        //     forwading_and_reorder_with_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        // }

        // fs::write("./pipeline_files/only_nops.txt", &only_nops_str)
        //     .expect("Failed to write only_nops_str to file");

        // fs::write(
        //     "./pipeline_files/forwarding_with_nops.txt",
        //     &forwarding_with_nops_str,
        // )
        // .expect("Failed to write forwarding_with_nops_str to file");

        // fs::write(
        //     "./pipeline_files/reorder_with_only_nops.txt",
        //     &reorder_with_only_nops_str,
        // )
        // .expect("Failed to write reorder_with_only_nops to file");

        // fs::write(
        //     "./pipeline_files/forwading_and_reorder_with_nops.txt",
        //     &forwading_and_reorder_with_nops_str,
        // )
        // .expect("Failed to write forwading_and_reorder_with_nops to file");
        // End: Implement techniques

        // Start: Calc Performance
        let tclock = performance_calculator_pipeline_dto.tclock;

        // End: Calc Performance

        Ok(String::from("TODO"))
    }
}

fn only_nops(instructions: Vec<Instruction>) -> Vec<Instruction> {
    // Considerar que não há nenhuma solução em hardware para conflitos e incluir NOPs, quando necessário, para evitar o conflito de dados.
    let mut nop_counter = 0;

    let mut instructions_with_nops = vec![instructions[0].clone()];

    for (mut index, curent_inst) in instructions[1..].iter().enumerate() {
        instructions_with_nops.push(curent_inst.clone());
        let prev_inst = instructions[index].clone();
        index += 1; // Adiciona 1 no index para continuar na ordem

        // Instruções de escrita (Possuem RD): U, J, I, L, R
        // Instruções de leitura (Somente RS1): I, L
        // Instruções de leitura (Possuem RS1 e RS2): B, S, R

        // Check for WAR hazards (Escrita-apos-Leitura) - OK
        // ha um conflito WAR, onde uma instrucao tenta escrever em um registrador que esta sendo lido por uma instrucao posterior.
        // Inst atual === escrita ----- Inst anterior === leitura
        match prev_inst.clone().get_opcode() {
            // Somente RS1
            OpCodeType::I(_) | OpCodeType::L(_) => {
                match curent_inst.clone().get_opcode() {
                    OpCodeType::I(_)
                    | OpCodeType::L(_)
                    | OpCodeType::J(_)
                    | OpCodeType::R(_)
                    | OpCodeType::U(_) => {
                        if prev_inst.clone().get_rs1() == curent_inst.clone().get_rd() {
                            // Insert NOP before the conflicted instruction
                            let nop = Instruction::new(NOP_INST);
                            instructions_with_nops.insert(index, nop.clone());
                            instructions_with_nops.insert(index, nop);
                            nop_counter += 2;
                            println!("WAR Hazard");
                            continue;
                        }
                    }
                    _ => (),
                }
            }

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
                            // Insert NOP before the conflicted instruction
                            let nop = Instruction::new(NOP_INST);
                            instructions_with_nops.insert(index, nop.clone());
                            instructions_with_nops.insert(index, nop);
                            nop_counter += 2;
                            println!("WAR Hazard");
                            continue;
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        // Check for WAW hazards (Escrita-apos-Escrita) - OK
        // o conflito e no WAW, onde duas instrucoes tentam escrever no mesmo registrador em uma ordem incorreta.
        // Inst atual === escrita ----- Inst anterior === escrita
        match prev_inst.clone().get_opcode() {
            OpCodeType::I(_)
            | OpCodeType::L(_)
            | OpCodeType::J(_)
            | OpCodeType::R(_)
            | OpCodeType::U(_)
            | OpCodeType::S(_) => {
                match curent_inst.clone().get_opcode() {
                    OpCodeType::I(_)
                    | OpCodeType::L(_)
                    | OpCodeType::J(_)
                    | OpCodeType::R(_)
                    | OpCodeType::U(_)
                    | OpCodeType::S(_) => {
                        if prev_inst.clone().get_rd() == curent_inst.clone().get_rd() {
                            // Insert NOP before the conflicted instruction
                            let nop = Instruction::new(NOP_INST);
                            instructions_with_nops.insert(index + nop_counter, nop.clone());
                            instructions_with_nops.insert(index + nop_counter, nop);
                            nop_counter += 2;
                            println!("WAW Hazard");
                            continue;
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        // Check for RAW hazards (Leitura-apos-Escrita) - OK
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
                        let nop = Instruction::new(NOP_INST);
                        instructions_with_nops.insert(index + nop_counter, nop.clone());
                        instructions_with_nops.insert(index + nop_counter, nop);
                        nop_counter += 2;
                        println!("RAW Hazard");
                        continue;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    println!("Only NOPs: {}", nop_counter);
    instructions_with_nops
}

fn forwading_with_nops(instructions: Vec<Instruction>) -> Vec<Instruction> {
    // Considerar que foi implementada a técnica de forwarding e inserir NOPs, quando necessário, para evitar conflito de dados.

    // Somente inserir nops para instrucoes de formato L
    let mut nop_counter = 0;

    let mut forwading_with_nops = vec![instructions[0].clone()];
    for (mut index, curent_inst) in instructions[1..].iter().enumerate() {
        let prev_inst = instructions[index].clone();

        index += 1; // Adiciona 1 no index para continuar na ordem
        forwading_with_nops.push(curent_inst.clone());

        match prev_inst.clone().get_opcode() {
            OpCodeType::L(_) => {
                if prev_inst.clone().get_rd() == curent_inst.clone().get_rd() {
                    // Insert NOP before the current instruction
                    let nop = Instruction::new(NOP_INST);
                    forwading_with_nops.insert(index + nop_counter, nop);
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

fn reorder_with_only_nops(instructions: Vec<Instruction>) -> Vec<Instruction> {
    // Considerar que não há nenhuma solução em hardware para conflitos e quando possível reordenar as instruções e quando não for possível inserir NOPs, para evitar conflito de dados.
    //     a. Por exemplo, é possível que o programa não tenha nenhuma instrução, a diante no código, para ser reordenada.

    let only_nops = only_nops(instructions.clone());
    let mut reorder_with_only_nops: Vec<Instruction> = vec![only_nops[0].clone()];

    let mut nop_counter = 0;

    // Nao reordena a primeira instrucao e inst do formato L e B
    for (mut index, curent_inst) in only_nops[1..].iter().enumerate() {
        let prev_inst = only_nops[index].clone();
        index += 1;

        if curent_inst.clone().get_full_inst() == NOP_INST {
            // Se a inst for NOP, nao faz nada
            reorder_with_only_nops.push(curent_inst.clone());
            nop_counter += 1;
            continue;
        }

        match prev_inst.clone().get_opcode() {
            OpCodeType::L(_) | OpCodeType::B(_) => {
                reorder_with_only_nops.push(curent_inst.clone());
                continue;
            } // Nao faz nada
            _ => {
                match curent_inst.clone().get_opcode() {
                    OpCodeType::L(_) | OpCodeType::B(_) => {
                        reorder_with_only_nops.push(curent_inst.clone());
                        continue;
                    } // Nao faz nada,
                    _ => {
                        // Verificar se pode reordenar
                        if !check_for_hazard(only_nops.clone(), index) {
                            // Pegar o último NOP
                            for inst_nop in only_nops[0..=index].into_iter().rev() {
                                match inst_nop.clone().get_opcode() {
                                    OpCodeType::B(_) | OpCodeType::L(_) => {
                                        break;
                                    }
                                    _ => {
                                        if inst_nop.clone().get_full_inst() == NOP_INST {
                                            println!(
                                                "Reversed: {}",
                                                inst_nop.clone().get_full_inst()
                                            );
                                            reorder_with_only_nops.push(curent_inst.clone());
                                            nop_counter -= 1;
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            reorder_with_only_nops.push(curent_inst.clone());
                        }
                    }
                }
            }
        }
    }

    println!("Reorder With NOPs: {}", nop_counter);

    reorder_with_only_nops
}

fn forwading_and_reorder_with_nops(instructions: Vec<Instruction>) -> Vec<Instruction> {
    // Considerar que foi implementada a técnica de forwarding e quando possível reordenar as instruções e quando não for possível inserir NOPs, para evitar conflito de dados.

    let forwading_with_nops = forwading_with_nops(instructions.clone());
    let mut forwading_and_reorder_with_nops: Vec<Instruction> =
        vec![forwading_with_nops[0].clone()];

    let mut nop_counter = 0;

    // Nao reordena a primeira instrucao e inst do formato L e B
    for (mut index, curent_inst) in forwading_with_nops[1..].iter().enumerate() {
        let prev_inst = forwading_with_nops[index].clone();
        index += 1;

        if curent_inst.clone().get_full_inst() == NOP_INST {
            // Se a inst for NOP, nao faz nada
            forwading_and_reorder_with_nops.push(curent_inst.clone());
            nop_counter += 1;
            continue;
        }

        match prev_inst.clone().get_opcode() {
            OpCodeType::L(_) | OpCodeType::B(_) => {
                forwading_and_reorder_with_nops.push(curent_inst.clone());
                continue;
            } // Nao faz nada
            _ => {
                match curent_inst.clone().get_opcode() {
                    OpCodeType::L(_) | OpCodeType::B(_) => {
                        forwading_and_reorder_with_nops.push(curent_inst.clone());
                        continue;
                    } // Nao faz nada,
                    _ => {
                        // Verificar se pode reordenar
                        if !check_for_hazard(forwading_with_nops.clone(), index) {
                            // Pegar o último NOP
                            for inst_nop in forwading_with_nops[0..=index].into_iter().rev() {
                                match inst_nop.clone().get_opcode() {
                                    OpCodeType::B(_) | OpCodeType::L(_) => {
                                        break;
                                    }
                                    _ => {
                                        if inst_nop.clone().get_full_inst() == NOP_INST {
                                            println!(
                                                "Reversed: {}",
                                                inst_nop.clone().get_full_inst()
                                            );
                                            forwading_and_reorder_with_nops
                                                .push(curent_inst.clone());
                                            nop_counter -= 1;
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            forwading_and_reorder_with_nops.push(curent_inst.clone());
                        }
                    }
                }
            }
        }
    }

    println!("Forwarding and Reorder With NOPs: {}", nop_counter);

    forwading_and_reorder_with_nops
}

fn check_for_hazard(instructions: Vec<Instruction>, current_index: usize) -> bool {
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

/////////// REFACTORED
///
///
///
fn only_nops_test(instructions: Vec<Instruction>) -> Vec<Instruction> {
    // Considerar que não há nenhuma solução em hardware para conflitos e incluir NOPs, quando necessário, para evitar o conflito de dados.
    let mut nop_counter = 0;
    let mut instructions_with_nops = vec![];

    let hazards = check_for_hazards(instructions.clone());

    instructions_with_nops
}
