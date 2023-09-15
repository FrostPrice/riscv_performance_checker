use actix_web::{http::StatusCode, web};

use crate::{
    config::db::Pool,
    models::organization::{Organization, OrganizationDTO},
    utils::error::ServiceError,
};

pub async fn find_by_id(id: String, pool: &web::Data<Pool>) -> Result<Organization, ServiceError> {
    match Organization::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(organization) => Ok(organization),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}

pub async fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Organization>, ServiceError> {
    match Organization::find_all(&mut pool.get().unwrap()) {
        Ok(organizations) => Ok(organizations),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}

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

pub async fn update(
    id: String,
    organization_dto: OrganizationDTO,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match Organization::update(id, organization_dto, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}

pub async fn delete(id: String, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Organization::delete(id, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    }
}
