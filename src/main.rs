use std::{env, io};

use actix_web::{web, App, HttpServer};

mod api;
mod config;
mod models;
mod performance_calculator;
mod riscv_core;
mod schema;
mod services;
mod utils;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    let pool = config::db::migrate_and_config_db(&db_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .configure(config::app::config_services)
    })
    .bind(&app_url)?
    .run()
    .await
}

// ################################
// .text

// 	addi s0, zero, 5
// 	addi s1, zero, 10

// for:
// 	beq  t0, s0, fim
// 	add  s1, s1, t0
// 	addi t0, t0, 1
// 	jal  zero, for
// fim:
// ################################

///// TODO:
/*
Solicitar input de informações de Ciclo por instrução de 2 organizações diferente - OK
Solicitar input de informação de clock (Tempo de relógio) - OK
Calcular o desempenho de ambas as organizações - OK
Informar qual a organização mais rápida (ou mais lenta), e o quao mais rapida (ou lenta) ela é - OK
Contar quantidade de instruções executadas - OK
O teste ira usar o monociclo e multiciclo - NOK
*/


///////////// TODOS:
/// 1. (Bolha) Considerar que não há nenhuma solução em hardware para conflitos e incluir NOPs, quando necessário, para evitar o conflito de dados.
/// 2. Considerar que foi implementada a técnica de forwarding e inserir NOPs, quando necessário, para evitar conflito de dados.
/// 3. Considerar que não há nenhuma solução em hardware para conflitos e quando possível reordenar as instruções e quando não for possível inserir NOPs, para evitar conflito de dados.
        // a. Por exemplo, é possível que o programa não tenha nenhuma instrução, a diante no código, para ser reordenada.
/// 4. Considerar que foi implementada a técnica de forwarding e quando possível reordenar as instruções e quando não for possível inserir NOPs, para evitar conflito de dados.