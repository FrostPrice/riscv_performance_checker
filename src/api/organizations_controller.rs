use actix_web::{web, HttpResponse, Result};

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
