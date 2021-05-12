mod database;

use actix_web::{
    get,
    web::{self, Data},
    App, HttpServer, Responder,
};
use dotenv::dotenv;
use actix_web::{middleware, HttpResponse};
use chrono::NaiveDate;
use database::Database;
use env_logger::Env;
use itertools::Itertools;
use log::info;
use std::{env, sync::Mutex};
use tokio_retry::strategy::FibonacciBackoff;
use tokio_retry::Retry;

#[get("/{user}/{item}/{year}/{month}/{day}/{amount}")]
async fn index(
    path: web::Path<(u32, u32, i32, u32, u32, u32)>,
    database: web::Data<Mutex<Database>>,
) -> impl Responder {
    let (user, item, year, month, day, amount) = path.into_inner();
    let date = NaiveDate::from_ymd(year, month, day);

    match database
        .lock()
        .unwrap()
        .reduce_stock(user, item, &date, amount)
        .await
    {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e.to_string())),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let scylla_hosts_value = env::var("SCYLLA_HOSTS").expect("SCYLLA_HOSTS must be set");
    let scylla_hosts = scylla_hosts_value.split(",").collect_vec();

    info!("Initializing services");
    let database = Retry::spawn(FibonacciBackoff::from_millis(5_000).take(5), || async {
        info!("Connecting to database");
        Database::new(&scylla_hosts[..])
            .await
            .map(Mutex::new)
            .map(Data::new)
    })
    .await
    .expect("Error connecting to database");

    info!("Starting server");

    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
