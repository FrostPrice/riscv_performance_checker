use actix_web::{http::StatusCode, web};

use crate::{
    config::db::Pool,
    performance_calculator::performance_calculator::{
        PerformanceCalculator, PerformanceCalculatorDTO,
    },
    utils::error::ServiceError,
};

pub async fn calc(
    performance_calculator_dto: PerformanceCalculatorDTO,
    pool: &web::Data<Pool>,
) -> Result<PerformanceCalculator, ServiceError> {
    match PerformanceCalculator::calc(performance_calculator_dto, &mut pool.get().unwrap()) {
        Ok(performance) => Ok(performance),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}
