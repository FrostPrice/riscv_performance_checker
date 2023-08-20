use actix_web::{http::StatusCode, web};

use crate::{
    config::db::Pool,
    models::organization::{Organization, OrganizationDTO},
    utils::error::ServiceError,
};

pub async fn insert(
    organization_dto: OrganizationDTO,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match Organization::insert(organization_dto, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}
