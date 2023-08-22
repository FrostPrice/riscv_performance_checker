use crate::{
    config::db::Pool, models::response::ResponseBody,
    performance_calculator::performance_calculator::PerformanceCalculatorDTO,
    services::performance_calculator_service, utils::constants,
};
use actix_web::{web, HttpResponse, Result};

// GET api/performance_calculator
pub async fn calc(
    performance_calculator_dto: web::Json<PerformanceCalculatorDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match performance_calculator_service::calc(performance_calculator_dto.0, &pool).await {
        Ok(performance) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, performance)))
        }
        Err(err) => Ok(err.response()),
    }
}
