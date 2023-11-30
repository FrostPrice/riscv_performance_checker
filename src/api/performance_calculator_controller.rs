use actix_web::{HttpResponse, Result, web};

use crate::{
    config::db::Pool,
    models::response::ResponseBody,
    performance_calculator::{
        monocycle_calculator::PerformanceCalculatorDTO,
        pipeline_calculator::PerformanceCalculatorPipelineDTO,
    },
    services::performance_calculator_service,
    utils::constants,
};

// GET api/performance_calculator/calc_monocycle
pub async fn calc_monocycle(
    performance_calculator_dto: web::Json<PerformanceCalculatorDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match performance_calculator_service::calc_monocycle(performance_calculator_dto.0, &pool).await
    {
        Ok(performance) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, performance)))
        }
        Err(err) => Ok(err.response()),
    }
}

// GET api/performance_calculator/calc_pipeline
pub async fn calc_pipeline(
    performance_calculator_pipeline_dto: web::Json<PerformanceCalculatorPipelineDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match performance_calculator_service::calc_pipeline(
        performance_calculator_pipeline_dto.0,
        &pool,
    )
    .await
    {
        Ok(performance) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, performance)))
        }
        Err(err) => Ok(err.response()),
    }
}
