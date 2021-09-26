mod database;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::{middleware, HttpResponse};
use chrono::NaiveDate;
use database::error::Kind;
use database::Database;
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use std::env;

#[get("/item/{item_id}/{year}/{month}/{day}/{quantity}")]
async fn update_stock(
    path: web::Path<(i32, i32, u32, u32, i32)>,
    database: web::Data<Database>,
) -> impl Responder {
    let (item_id, year, month, day, quantity) = path.into_inner();
    let date = NaiveDate::from_ymd(year, month, day);

    match database.reduce_stock(item_id, date, quantity).await {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[get("/item")]
async fn get_items(database: web::Data<Database>) -> impl Responder {
    match database.get_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[get("/item/{item_id}")]
async fn get_item(path: web::Path<i32>, database: web::Data<Database>) -> impl Responder {
    let item_id = path.into_inner();

    match database.get_item(item_id).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => match e.0 {
            Kind::ItemNotFound => HttpResponse::NotFound().body("Item not found"),
            _ => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        },
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
            .app_data(web::Data::new(database.clone()))
            .wrap(middleware::NormalizePath::trim())
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .service(update_stock)
            .service(get_items)
            .service(get_item)
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
