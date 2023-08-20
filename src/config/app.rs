use crate::api::*;
use actix_web::web;
use log::info;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");

    cfg.service(
        web::scope("/api")
            .service(web::scope("/organizations").service(
                web::resource("").route(web::post().to(organizations_controller::insert)),
            )),
    );
}
