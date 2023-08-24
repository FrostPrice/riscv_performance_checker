use serde::{Deserialize, Serialize};

use crate::{
    config::db::Connection,
    models::{bin_file::BinFile, organization::Organization},
    riscv_core::{self, instruction::OpCodeType},
};

#[derive(Serialize, Deserialize)]

pub struct PerformanceCalculator {
    pub basic_information: BasicInformation,
    pub result: Result,
}

#[derive(Serialize, Deserialize)]
pub struct BasicInformation {
    pub organization_a_name: String,
    pub organization_a_clock_time: f32,
    pub organization_b_name: String,
    pub organization_b_clock_time: f32,
    pub bin_file_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub total_cicles_organization_a: f32,
    pub total_cicles_organization_b: f32,
    pub average_cpi_organization_a: f32,
    pub average_cpi_organization_b: f32,
    // pub execution_time_organization_a: f32,
    // pub execution_time_organization_b: f32,
    pub best_performance: String,
}

#[derive(Serialize, Deserialize)]
pub struct PerformanceCalculatorDTO {
    pub organization_a_name: String,
    pub organization_b_name: String,
    pub bin_file_name: String,
}

impl PerformanceCalculator {
    pub fn calc(
        performance_calculator_dto: PerformanceCalculatorDTO,
        conn: &mut Connection,
    ) -> actix_web::Result<PerformanceCalculator, String> {
        // Start: Get info from database
        let organization_a = match Organization::find_by_id(
            performance_calculator_dto.organization_a_name.clone(),
            conn,
        ) {
            Ok(organization) => organization,
            Err(_) => {
                return Err(format!(
                    "Organization {} not found",
                    performance_calculator_dto.organization_a_name
                ))
            }
        };

        let organization_b = match Organization::find_by_id(
            performance_calculator_dto.organization_b_name.clone(),
            conn,
        ) {
            Ok(organization) => organization,
            Err(_) => {
                return Err(format!(
                    "Organization {} not found",
                    performance_calculator_dto.organization_b_name
                ))
            }
        };

        let bin_file =
            match BinFile::find_by_id(performance_calculator_dto.bin_file_name.clone(), conn) {
                Ok(bin_file) => bin_file,
                Err(_) => {
                    return Err(format!(
                        "Bin file {} not found",
                        performance_calculator_dto.bin_file_name
                    ))
                }
            };
        // End: Get info from database

        // Start: calculating instruction info
        let total_instructions = bin_file.file.lines().count();
        let mut total_cicles_a: f32 = 0.0;
        let mut total_cicles_b: f32 = 0.0;

        for line in bin_file.file.trim().lines() {
            let inst = riscv_core::instruction::Instruction::new(line);

            let opcode = inst.clone().get_opcode();

            let cpi_a: &f32;
            let cpi_b: &f32;

            match opcode {
                OpCodeType::R(_) => {
                    cpi_a = &organization_a.cpi_instruction_r;
                    cpi_b = &organization_b.cpi_instruction_r;
                }
                OpCodeType::I(_) => {
                    cpi_a = &organization_a.cpi_instruction_i;
                    cpi_b = &organization_b.cpi_instruction_i;
                }
                OpCodeType::L(_) => {
                    cpi_a = &organization_a.cpi_instruction_l;
                    cpi_b = &organization_b.cpi_instruction_l;
                }
                OpCodeType::S(_) => {
                    cpi_a = &organization_a.cpi_instruction_s;
                    cpi_b = &organization_b.cpi_instruction_s;
                }
                OpCodeType::B(_) => {
                    cpi_a = &organization_a.cpi_instruction_b;
                    cpi_b = &organization_b.cpi_instruction_b;
                }
                OpCodeType::U(_) => {
                    cpi_a = &organization_a.cpi_instruction_u;
                    cpi_b = &organization_b.cpi_instruction_u;
                }
                OpCodeType::J(_) => {
                    cpi_a = &organization_a.cpi_instruction_j;
                    cpi_b = &organization_b.cpi_instruction_j;
                }
            }

            total_cicles_a += cpi_a;
            total_cicles_b += cpi_b;
        }

        // CPI = total_cycles (with acordingly Instruction cycle) / total_instructions
        let average_cpi_a = total_cicles_a / total_instructions as f32;
        let average_cpi_b = total_cicles_b / total_instructions as f32;

        // Texec = Total Instructions * CPI * TClock
        let execution_time_a = total_cicles_a * average_cpi_a * organization_a.clock; // In seconds
        let execution_time_b = total_cicles_b * average_cpi_b * organization_b.clock; // In seconds

        // Texec = (Total Instructions * CPI) / FClock
        // let execution_time_a = (total_cicles_a * average_cpi_a) / organization_a.clock; // In seconds
        // let execution_time_b = (total_cicles_b * average_cpi_b) / organization_b.clock; // In seconds

        // Performance = Texec(GreaterValue) / Texec(LesserValue)
        let best_performance: String;
        if execution_time_a < execution_time_b {
            best_performance = format!(
                "A organizacao {} eh {} vezes mais rapida que a organizacao {}",
                organization_a.id,
                execution_time_b / execution_time_a,
                organization_b.id
            );
        } else if execution_time_a > execution_time_b {
            best_performance = format!(
                "A organizacao {} eh {} vezes mais rapida que a organizacao {}",
                organization_b.id,
                execution_time_a / execution_time_b,
                organization_a.id
            );
        } else {
            best_performance = format!(
                "As organizacoes {} e {} sao igualmente rapidas",
                organization_a.id, organization_b.id
            );
        }
        // End: calculating instruction info

        // Start: function return
        Ok(PerformanceCalculator {
            basic_information: BasicInformation {
                organization_a_name: organization_a.id,
                organization_a_clock_time: organization_a.clock,
                organization_b_name: organization_b.id,
                organization_b_clock_time: organization_b.clock,
                bin_file_name: bin_file.id,
            },
            result: Result {
                total_cicles_organization_a: total_cicles_a,
                total_cicles_organization_b: total_cicles_b,
                average_cpi_organization_a: average_cpi_a,
                average_cpi_organization_b: average_cpi_b,
                // execution_time_organization_a: execution_time_a,
                // execution_time_organization_b: execution_time_b,
                best_performance,
            },
        })
        // End: function return
    }
}
