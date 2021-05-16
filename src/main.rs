mod database;

use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::{middleware, HttpResponse};
use chrono::NaiveDate;
use database::Database;
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use std::env;

#[get("/{item}/{year}/{month}/{day}/{quantity}")]
async fn index(
    path: web::Path<(i32, i32, u32, u32, i32)>,
    database: web::Data<Database>,
) -> impl Responder {
    let (item, year, month, day, quantity) = path.into_inner();
    let date = NaiveDate::from_ymd(year, month, day);

    match database.reduce_stock(item, date, quantity).await {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    info!("Initializing services");
    let database = initialize_database().await;

    info!("Starting server");
    HttpServer::new(move || {
        App::new()
            .data(database.clone())
            .wrap(middleware::Logger::default())
            .service(index)
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
