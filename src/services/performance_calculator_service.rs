use actix_web::{http::StatusCode, web};

use crate::{
    config::db::Pool,
    performance_calculator::{
        monocycle_calculator::{PerformanceCalculator, PerformanceCalculatorDTO},
        pipeline_calculator::PerformanceCalculatorPipelineDTO,
    },
    utils::error::ServiceError,
};

pub async fn calc_monocycle(
    performance_calculator_dto: PerformanceCalculatorDTO,
    pool: &web::Data<Pool>,
) -> Result<PerformanceCalculator, ServiceError> {
    match crate::performance_calculator::monocycle_calculator::PerformanceCalculator::calc_monocycle(
        performance_calculator_dto,
        &mut pool.get().unwrap(),
    ) {
        Ok(performance) => Ok(performance),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}

pub async fn calc_pipeline(
    performance_calculator_pipeline_dto: PerformanceCalculatorPipelineDTO,
    pool: &web::Data<Pool>,
) -> Result<String, ServiceError> {
    match crate::performance_calculator::pipeline_calculator::PerformanceCalculator::calc_pipeline(
        performance_calculator_pipeline_dto,
        &mut pool.get().unwrap(),
    ) {
        Ok(performance) => Ok(performance),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}
