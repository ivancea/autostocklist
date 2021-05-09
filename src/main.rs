use std::error::Error;
use chrono::NaiveDate;
use env_logger::Env;
use log::info;
use actix_web::{HttpResponse, middleware};
use actix_web::{get, web, App, HttpServer, Responder};
use scylla::{batch::Consistency, SessionBuilder};

#[get("/{user}/{item}/{year}/{month}/{day}/{difference}")]
async fn index(path: web::Path<(i32, i32, i32, u32, u32, i32)>) -> impl Responder {
    let (user, item, year, month, day, difference) = path.into_inner();
    let date = NaiveDate::from_ymd(year, month, day);

    match insert(user, item, &date, difference).await {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e.to_string())),
    }
}

async fn insert(
    user: i32,
    item: i32,
    date: &NaiveDate,
    difference: i32,
) -> Result<(), Box<dyn Error>> {
    let session = SessionBuilder::new()
        .known_node("db1:9042")
        .known_node("db2:9042")
        .known_node("db3:9042")
        .build()
        .await?;

    let mut prepared = session
        .prepare(
            r#"
            UPDATE stock.stock_movements
            SET difference = difference + ?
            WHERE user = ?
              AND item = ?
              AND date = ?;
        "#,
        )
        .await?;

    prepared.set_consistency(Consistency::Quorum);

    session
        .execute(&prepared, (difference, user, item, date))
        .await?;

    Ok(())
}

#[get("/")]
async fn test() -> impl Responder {
    "Test"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Starting server");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(test)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
