use actix_web::web;
use log::info;

use crate::api::*;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");

    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/organizations")
                    .service(
                        web::resource("")
                            .route(web::get().to(organizations_controller::find_all))
                            .route(web::post().to(organizations_controller::insert)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(organizations_controller::find_by_id))
                            .route(web::put().to(organizations_controller::update))
                            .route(web::delete().to(organizations_controller::delete)),
                    ),
            )
            .service(
                web::scope("/bin_files")
                    .service(web::resource("").route(web::get().to(bin_files_controller::find_all)))
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(bin_files_controller::find_by_id))
                            .route(web::put().to(bin_files_controller::insert))
                            .route(web::put().to(bin_files_controller::update))
                            .route(web::delete().to(bin_files_controller::delete)),
                    ),
            )
            .service(
                web::scope("/performance_calculator")
                    .service(
                        web::resource("/calc_monocycle").route(
                            web::get().to(performance_calculator_controller::calc_monocycle),
                        ),
                    )
                    .service(
                        web::resource("/calc_pipeline")
                            .route(web::get().to(performance_calculator_controller::calc_pipeline)),
                    ),
            ),
    );
}
