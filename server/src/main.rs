mod database;
mod dtos;
mod http_configuration;
mod services;

use actix_cors::Cors;
use actix_web::middleware;
use actix_web::{web, App, HttpServer};
use database::Database;
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use services::{item_service::ItemService, stock_service::StockService};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    info!("Initializing services");
    let database = initialize_database().await;
    let item_service = ItemService::new(database.clone());
    let stock_service = StockService::new(database.clone());

    info!("Starting server");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(item_service.clone()))
            .app_data(web::Data::new(stock_service.clone()))
            .wrap(middleware::NormalizePath::trim())
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .configure(http_configuration::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn initialize_database() -> Database {
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_port_string = env::var("DB_PORT").unwrap_or("5432".to_string());
    let db_port = u16::from_str_radix(db_port_string.as_ref(), 10)
        .expect("DB_PORT must be a valid port number");
    let db_database = env::var("DB_DATABASE").expect("DB_DATABASE must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");

    info!("Connecting to database");
    Database::new(&db_host, db_port, &db_database, &db_user, &db_password)
        .await
        .expect("Error connecting to database")
}
