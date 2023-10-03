use std::fs;

use serde::{Deserialize, Serialize};

use crate::{
    config::db::Connection,
    models::{bin_file::BinFile, organization::Organization},
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
    pub results: Vec<TechniqueResult>,
}

#[derive(Serialize, Deserialize)]
pub struct BasicInformation {
    pub organization_name: String,
    pub organization_clock_time: f32,
    pub bin_file_name: String,
    pub best_performance: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TechniqueResult {
    pub technique_name: String,
    pub total_cicles: f32,
    pub cicles_diference: f32,
    pub average_cpi: f32,
    pub execution_time: f32,
    pub performance: f32,
}

#[derive(Serialize, Deserialize)]
pub struct PerformanceCalculatorPipelineDTO {
    pub organization_name: String,
    pub bin_file_name: String,
}

impl PerformanceCalculator {
    pub fn calc_pipeline(
        performance_calculator_pipeline_dto: PerformanceCalculatorPipelineDTO,
        conn: &mut Connection,
    ) -> actix_web::Result<PerformanceCalculator, String> {
        // Start: Get info from database
        let organization: Organization = match Organization::find_by_id(
            performance_calculator_pipeline_dto
                .organization_name
                .clone(),
            conn,
        ) {
            Ok(organization) => organization,
            Err(_) => {
                return Err(format!(
                    "Organization {} not found",
                    performance_calculator_pipeline_dto.organization_name
                ))
            }
        };

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
        for i in only_nops.clone() {
            only_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        }

        let forwading_with_nops = forwading_with_nops(instructions.clone());
        let mut forwarding_with_nops_str = String::new();
        for i in forwading_with_nops.clone() {
            forwarding_with_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        }

        let reorder_with_only_nops = reorder_with_only_nops(instructions.clone());
        let mut reorder_with_only_nops_str = String::new();
        for i in reorder_with_only_nops.clone() {
            reorder_with_only_nops_str.push_str(&format!("{}\n", i.get_full_inst()));
        }

        let forwading_and_reorder_with_nops =
            forwarding_and_reorder_with_nops(instructions.clone());
        let mut forwading_and_reorder_with_nops_str = String::new();
        for i in forwading_and_reorder_with_nops.clone() {
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
        let results = Self::calc_techniques(
            organization.clone(),
            instructions,
            only_nops,
            forwading_with_nops,
            reorder_with_only_nops,
            forwading_and_reorder_with_nops,
        );

        // End: Calc Performance

        // Start: define best technique performance
        let mut sorted_results = results.clone();
        sorted_results.sort_by(|a, b| b.performance.partial_cmp(&a.performance).unwrap());

        let best_performance = sorted_results
            .iter()
            .map(|technique| technique.technique_name.clone())
            .collect();
        // End: define best technique performance

        let basic_information = BasicInformation {
            organization_name: organization.clone().id,
            organization_clock_time: organization.clone().clock,
            bin_file_name: performance_calculator_pipeline_dto.bin_file_name,
            best_performance,
        };

        Ok(PerformanceCalculator {
            basic_information,
            results,
        })
    }

    fn calc_techniques(
        organization: Organization,
        instructions: Vec<Instruction>,
        only_nops: Vec<Instruction>,
        forwading_with_nops: Vec<Instruction>,
        reorder_with_only_nops: Vec<Instruction>,
        forwading_and_reorder_with_nops: Vec<Instruction>,
    ) -> Vec<TechniqueResult> {
        // Elaborar uma analise de desempenho que avalie o sobrecusto em instrucoes da solucao, o tempo de execucao e o numero de ciclos de programa da solucao em Pipeline selecionada considerando um tempo de clock fornecido pelo usuario.
        let mut techniques_result: Vec<TechniqueResult> = Vec::new();

        // Calc from original
        // Start: calculating instruction info
        let total_instructions = instructions.len();
        let mut total_cicles: f32 = 0.0;

        for inst in instructions {
            let opcode = inst.clone().get_opcode();

            let cpi: &f32;

            match opcode {
                OpCodeType::R(_) => {
                    cpi = &organization.cpi_instruction_r;
                }
                OpCodeType::I(_) => {
                    cpi = &organization.cpi_instruction_i;
                }
                OpCodeType::L(_) => {
                    cpi = &organization.cpi_instruction_l;
                }
                OpCodeType::S(_) => {
                    cpi = &organization.cpi_instruction_s;
                }
                OpCodeType::B(_) => {
                    cpi = &organization.cpi_instruction_b;
                }
                OpCodeType::U(_) => {
                    cpi = &organization.cpi_instruction_u;
                }
                OpCodeType::J(_) => {
                    cpi = &organization.cpi_instruction_j;
                }
            }

            total_cicles += cpi;
        }

        // CPI = total_cycles (with acordingly Instruction cycle) / total_instructions
        let average_cpi = total_cicles / total_instructions as f32;

        // For when using cpu time
        // Texec = Total Instructions * CPI * TClock
        let execution_time = total_instructions as f32 * average_cpi * organization.clock; // In seconds

        // For when using cpu frequency
        // Texec = (Total Instructions * CPI) / FClock
        // let execution_time = (total_cicles * average_cpi) / organization.clock; // In seconds

        let performance = 1.0; // A performance do original sempre sera 1.0

        let original = TechniqueResult {
            technique_name: "original".to_string(),
            total_cicles,
            cicles_diference: 0.0,
            average_cpi,
            execution_time,
            performance,
        };
        // End: calculating instruction info

        // Calc from only_nops
        let only_nops = Self::calc_performance(
            organization.clone(),
            only_nops.clone(),
            total_cicles,
            execution_time,
            String::from("only_nops"),
        );

        // Calc from forwading_with_nops
        let forwading_with_nops = Self::calc_performance(
            organization.clone(),
            forwading_with_nops.clone(),
            total_cicles,
            execution_time,
            String::from("forwading_with_nops"),
        );

        // Calc from reorder_with_only_nops
        let reorder_with_only_nops = Self::calc_performance(
            organization.clone(),
            reorder_with_only_nops.clone(),
            total_cicles,
            execution_time,
            String::from("reorder_with_only_nops"),
        );

        // Calc from forwading_and_reorder_with_nops
        let forwading_and_reorder_with_nops = Self::calc_performance(
            organization.clone(),
            forwading_and_reorder_with_nops.clone(),
            total_cicles,
            execution_time,
            String::from("forwading_and_reorder_with_nops"),
        );

        techniques_result.push(original);
        techniques_result.push(only_nops);
        techniques_result.push(forwading_with_nops);
        techniques_result.push(reorder_with_only_nops);
        techniques_result.push(forwading_and_reorder_with_nops);

        techniques_result
    }

    fn calc_performance(
        organization: Organization,
        instructions: Vec<Instruction>,
        original_cicles: f32,
        original_exec_time: f32,
        technique_name: String,
    ) -> TechniqueResult {
        // Start: calculating instruction info
        let total_instructions = instructions.len();
        let mut total_cicles: f32 = 0.0;

        for inst in instructions {
            let opcode = inst.clone().get_opcode();

            let cpi: &f32;

            match opcode {
                OpCodeType::R(_) => {
                    cpi = &organization.cpi_instruction_r;
                }
                OpCodeType::I(_) => {
                    cpi = &organization.cpi_instruction_i;
                }
                OpCodeType::L(_) => {
                    cpi = &organization.cpi_instruction_l;
                }
                OpCodeType::S(_) => {
                    cpi = &organization.cpi_instruction_s;
                }
                OpCodeType::B(_) => {
                    cpi = &organization.cpi_instruction_b;
                }
                OpCodeType::U(_) => {
                    cpi = &organization.cpi_instruction_u;
                }
                OpCodeType::J(_) => {
                    cpi = &organization.cpi_instruction_j;
                }
            }

            total_cicles += cpi;
        }

        // CPI = total_cycles (with acordingly Instruction cycle) / total_instructions
        let average_cpi = total_cicles / total_instructions as f32;

        // For when using cpu time
        // Texec = Total Instructions * CPI * TClock
        let execution_time = total_instructions as f32 * average_cpi * organization.clock; // In seconds

        // For when using cpu frequency
        // Texec = (Total Instructions * CPI) / FClock
        // let execution_time = (total_cicles * average_cpi) / organization.clock; // In seconds

        let cicles_diference = total_cicles - original_cicles;

        let mut performance = 1.0;
        if original_exec_time > 0.0 {
            performance = original_exec_time / execution_time;
        }
        // End: calculating instruction info

        // Start: function return
        TechniqueResult {
            technique_name,
            cicles_diference,
            average_cpi,
            execution_time,
            performance,
            total_cicles,
        }
        // End: function return
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
            reorder_with_only_nops.push(current_inst.clone());

            if let Some(bool_insts) = can_reorder[index].clone() {
                if bool_insts[1] {
                    if index >= 3 {
                        let prev_index = index - 2;

                        reorder_with_only_nops.remove(prev_index);
                        reorder_with_only_nops.insert(prev_index, current_inst.clone());
                        reorder_with_only_nops.remove(index);

                        nop_counter -= 1;
                    }
                }
            }
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
    let can_reorder = check_for_reorder(forwading_with_nops.clone());

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
            forwarding_and_reorder_with_nops.push(current_inst.clone());

            if let Some(bool_insts) = can_reorder[index].clone() {
                if bool_insts[1] {
                    if index >= 3 {
                        let prev_index = index - 2;

                        let nops = forwarding_and_reorder_with_nops[prev_index].clone();
                        if nops.get_full_inst() == NOP_INST {
                            forwarding_and_reorder_with_nops.remove(prev_index);
                            forwarding_and_reorder_with_nops
                                .insert(prev_index, current_inst.clone());
                            forwarding_and_reorder_with_nops.remove(index);

                            nop_counter -= 1;
                        }
                    }
                }
            }
        }
    }

    println!("Forwarding and Reorder With NOPs: {}", nop_counter);

    forwarding_and_reorder_with_nops
}
