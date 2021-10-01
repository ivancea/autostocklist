use crate::database::Database;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::{post, web, Responder};
use chrono::{NaiveDate, Utc};
use serde::Deserialize;

pub fn configure(server: &mut ServiceConfig) {
    server.service(web::scope("/item/{item_id}").service(update_stock_loss));
}

#[derive(Deserialize)]
struct Date {
    day: u32,
    month: u32,
    year: i32,
}

#[derive(Deserialize)]
struct UpdateStockBody {
    quantity: i32,
    date: Option<Date>,
}

#[post("/loss")]
async fn update_stock_loss(
    path: web::Path<i32>,
    body: web::Json<UpdateStockBody>,
    database: web::Data<Database>,
) -> impl Responder {
    let item_id = path.into_inner();
    let quantity = body.quantity;
    let date = match &body.date {
        Some(date) => NaiveDate::from_ymd_opt(date.year, date.month, date.day),
        None => Some(Utc::now().naive_utc().date()),
    };

    if date.is_none() {
        return HttpResponse::BadRequest().body("Invalid date provided");
    }

    match database
        .update_stock_loss(item_id, date.unwrap(), quantity)
        .await
    {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
