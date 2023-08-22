use crate::api::*;
use actix_web::web;
use log::info;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");

    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/organizations")
                    .service(
                        web::resource("").route(web::post().to(organizations_controller::insert)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(organizations_controller::find_by_id)),
                    ),
            )
            .service(
                web::scope("/bin_files").service(
                    web::resource("/{id}")
                        .route(web::get().to(bin_files_controller::find_by_id))
                        .route(web::put().to(bin_files_controller::insert)),
                ),
            )
            .service(
                web::scope("/performance_calculator").service(
                    web::resource("/calc")
                        .route(web::get().to(performance_calculator_controller::calc)),
                ),
            ),
    );
}
