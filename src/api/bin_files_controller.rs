use actix_web::{
    web::{self, Bytes},
    HttpResponse, Result,
};

use crate::{
    config::db::Pool, models::response::ResponseBody, services::bin_files_service, utils::constants,
};

// GET api/bin_files/{id}
pub async fn find_by_id(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match bin_files_service::find_by_id(id.into_inner(), &pool).await {
        Ok(bin_file) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, bin_file)))
        }
        Err(err) => Ok(err.response()),
    }
}

// GET api/bin_files
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match bin_files_service::find_all(&pool).await {
        Ok(bin_files) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, bin_files)))
        }
        Err(err) => Ok(err.response()),
    }
}

// POST api/bin_files/{id}
pub async fn insert(
    id: web::Path<String>,
    bin_file_data: Bytes,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match bin_files_service::insert(id.to_string(), bin_file_data, &pool).await {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}

// PUT api/bin_files/{id}
pub async fn update(
    id: web::Path<String>,
    bin_file_data: Bytes,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match bin_files_service::update(id.to_string(), bin_file_data, &pool).await {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}

// DELETE api/bin_files/{id}
pub async fn delete(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match bin_files_service::delete(id.into_inner(), &pool).await {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}
