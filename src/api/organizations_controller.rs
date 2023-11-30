use actix_web::{HttpResponse, Result, web};

use crate::{
    config::db::Pool,
    models::{organization::OrganizationDTO, response::ResponseBody},
    services::organizations_service,
    utils::constants,
};

// GET api/organization/{id}
pub async fn find_by_id(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match organizations_service::find_by_id(id.into_inner(), &pool).await {
        Ok(organization) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, organization)))
        }
        Err(err) => Ok(err.response()),
    }
}

// GET api/organization
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match organizations_service::find_all(&pool).await {
        Ok(organizations) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, organizations)))
        }
        Err(err) => Ok(err.response()),
    }
}

// POST api/organization
pub async fn insert(
    organization_dto: web::Json<OrganizationDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match organizations_service::insert(organization_dto.0, &pool).await {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}

// PUT api/organization/{id}
pub async fn update(
    id: web::Path<String>,
    organization_dto: web::Json<OrganizationDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match organizations_service::update(id.into_inner(), organization_dto.0, &pool).await {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}

// DELETE api/organization/{id}
pub async fn delete(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match organizations_service::delete(id.into_inner(), &pool).await {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}
