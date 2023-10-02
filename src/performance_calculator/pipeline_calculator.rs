use std::{fs, ops::Index};

use serde::{Deserialize, Serialize};

use crate::{
    config::db::Connection,
    models::bin_file::BinFile,
    performance_calculator::data_hazard::{
        check_for_hazards, check_for_hazards_with_forwarding, check_for_reorder,
    },
    riscv_core::{
        self,
        instruction::{Instruction, OpCodeType},
    },
    utils::constants::NOP_INST,
};

use super::data_hazard::DataHazard;

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

        // Add instructions struct to Vector
        let mut instructions = Vec::<Instruction>::new();
        for line in bin_file.file.trim().lines() {
            let inst = riscv_core::instruction::Instruction::new(line);
            instructions.push(inst);
        }

        // Start: Execute techniques
        let only_nops = only_nops(instructions.clone());
        let mut only_nops_str = String::new();
        for i in only_nops {
            only_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        }

        let forwading_with_nops = forwading_with_nops(instructions.clone());
        let mut forwarding_with_nops_str = String::new();
        for i in forwading_with_nops {
            forwarding_with_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        }

        let reorder_with_only_nops = reorder_with_only_nops(instructions.clone());
        let mut reorder_with_only_nops_str = String::new();
        for i in reorder_with_only_nops {
            reorder_with_only_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        }

        let forwading_and_reorder_with_nops =
            forwarding_and_reorder_with_nops(instructions.clone());
        let mut forwading_and_reorder_with_nops_str = String::new();
        for i in forwading_and_reorder_with_nops {
            forwading_and_reorder_with_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        }
        // End: Execute techniques

        // Start: Write to files
        fs::write("./pipeline_files/only_nops.txt", &only_nops_str)
            .expect("Failed to write only_nops_str to file");

        fs::write(
            "./pipeline_files/forwarding_with_nops.txt",
            &forwarding_with_nops_str,
        )
        .expect("Failed to write forwarding_with_nops_str to file");

        fs::write(
            "./pipeline_files/reorder_with_only_nops.txt",
            &reorder_with_only_nops_str,
        )
        .expect("Failed to write reorder_with_only_nops to file");

        fs::write(
            "./pipeline_files/forwading_and_reorder_with_nops.txt",
            &forwading_and_reorder_with_nops_str,
        )
        .expect("Failed to write forwading_and_reorder_with_nops to file");
        // End: Write to files

        // Start: Calc Performance
        let tclock = performance_calculator_pipeline_dto.tclock;

        // End: Calc Performance

        Ok(String::from("TODO"))
    }
}

fn only_nops(instructions: Vec<Instruction>) -> Vec<Instruction> {
    // Considerar que não há nenhuma solução em hardware para conflitos e incluir NOPs, quando necessário, para evitar o conflito de dados.
    let mut nop_counter = 0;
    let mut instructions_with_nops = vec![];
    let nop = Instruction::new(NOP_INST);

    let hazards = check_for_hazards(instructions.clone());

    for (index, inst) in instructions.iter().enumerate() {
        instructions_with_nops.push(inst.clone());

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

        let mut _highest_nops = 0;
        if war_nops >= waw_nops && war_nops >= raw_nops {
            _highest_nops = war_nops;
        } else if waw_nops >= raw_nops {
            _highest_nops = waw_nops;
        } else {
            _highest_nops = raw_nops;
        }

        for _ in 0.._highest_nops {
            instructions_with_nops.push(nop.clone());
            nop_counter += 1;
        }
    }

    println!("NOP: {}", nop_counter);

    instructions_with_nops
}

fn forwading_with_nops(instructions: Vec<Instruction>) -> Vec<Instruction> {
    // Considerar que foi implementada a técnica de forwarding e inserir NOPs, quando necessário, para evitar conflito de dados.
    // Somente inserir nops para instrucoes de formato L

    let mut nop_counter = 0;
    let mut forwading_with_nops = vec![];
    let nop = Instruction::new(NOP_INST);

    let hazards = check_for_hazards_with_forwarding(instructions.clone());
    for (index, inst) in instructions.iter().enumerate() {
        forwading_with_nops.push(inst.clone());

        let nops = hazards[index];

        for _ in 0..nops.clone() {
            forwading_with_nops.push(nop.clone());
            nop_counter += 1;
        }
    }

    println!("Forwading with NOPs: {}", nop_counter);

    forwading_with_nops
}

fn reorder_with_only_nops(instructions: Vec<Instruction>) -> Vec<Instruction> {
    // Considerar que não há nenhuma solução em hardware para conflitos e quando possível reordenar as instruções e quando não for possível inserir NOPs, para evitar conflito de dados.
    //     a. Por exemplo, é possível que o programa não tenha nenhuma instrução, a diante no código, para ser reordenada.

    let mut nop_counter = 0;
    let mut reorder_with_only_nops: Vec<Instruction> = vec![];
    let only_nops = only_nops(instructions.clone());

    let can_reorder = check_for_reorder(only_nops.clone());

    // Nao reordena a primeira instrucao e inst do formato J e B
    for (index, current_inst) in only_nops.iter().enumerate() {
        if current_inst.clone().get_full_inst() == NOP_INST {
            reorder_with_only_nops.push(current_inst.clone());
            nop_counter += 1;
        } else {
            if let Some(bool_insts) = can_reorder[index].clone() {
                if bool_insts[1] {
                    if index - 2 > 1 {
                        reorder_with_only_nops.remove(index - 2);
                        reorder_with_only_nops.insert(index - 2, current_inst.clone());
                        nop_counter -= 1;
                    }
                }
            }

            reorder_with_only_nops.push(current_inst.clone());
        }
    }

    println!("Reorder With NOPs: {}", nop_counter);

    reorder_with_only_nops
}

fn forwarding_and_reorder_with_nops(instructions: Vec<Instruction>) -> Vec<Instruction> {
    // Considerar que foi implementada a técnica de forwarding e quando possível reordenar as instruções e quando não for possível inserir NOPs, para evitar conflito de dados.

    let mut nop_counter = 0;
    let mut forwarding_and_reorder_with_nops: Vec<Instruction> = vec![];
    let forwading_with_nops = forwading_with_nops(instructions.clone());
    let can_reorder = check_for_reorder(instructions.clone());

    for (index, current_inst) in forwading_with_nops.iter().enumerate() {
        // Nao reordena inst do formato J e B
        match current_inst.clone().get_opcode() {
            OpCodeType::B(_) | OpCodeType::J(_) => {
                forwarding_and_reorder_with_nops.push(current_inst.clone());
                continue;
            }
            _ => (),
        }

        if current_inst.clone().get_full_inst() == NOP_INST {
            forwarding_and_reorder_with_nops.push(current_inst.clone());
            nop_counter += 1;
        } else {
            if let Some(bool_insts) = can_reorder[index].clone() {
                if bool_insts[1] {
                    if index - 2 > 1 {
                        let nops = forwarding_and_reorder_with_nops[index - 2].clone();
                        if nops.get_full_inst() == NOP_INST {
                            forwarding_and_reorder_with_nops.remove(index - 2);
                            forwarding_and_reorder_with_nops
                                .insert(index - 2, current_inst.clone());
                            nop_counter -= 1;
                        }
                    }
                }
            }

            forwarding_and_reorder_with_nops.push(current_inst.clone());
        }
    }

    println!("Forwarding and Reorder With NOPs: {}", nop_counter);

    forwarding_and_reorder_with_nops
}
