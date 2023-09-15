use actix_web::{
    http::StatusCode,
    web::{self, Bytes},
};

use crate::{config::db::Pool, models::bin_file::BinFile, utils::error::ServiceError};

pub async fn find_by_id(id: String, pool: &web::Data<Pool>) -> Result<BinFile, ServiceError> {
    match BinFile::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(bin_file) => Ok(bin_file),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}

pub async fn find_all(pool: &web::Data<Pool>) -> Result<Vec<BinFile>, ServiceError> {
    match BinFile::find_all(&mut pool.get().unwrap()) {
        Ok(bin_files) => Ok(bin_files),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}

pub async fn insert(
    id: String,
    bin_file_data: Bytes,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match BinFile::insert(id, bin_file_data, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}

pub async fn update(
    id: String,
    bin_file_data: Bytes,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match BinFile::update(id, bin_file_data, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}

pub async fn delete(id: String, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match BinFile::delete(id, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}
