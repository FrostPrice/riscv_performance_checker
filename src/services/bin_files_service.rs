use actix_web::{
    http::StatusCode,
    web::{self, Bytes},
};

use crate::{config::db::Pool, models::bin_file::BinFile, utils::error::ServiceError};

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
