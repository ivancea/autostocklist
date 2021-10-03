use crate::services::error::ServiceError;
use crate::services::item_service::ItemService;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::{post, web, Responder};
use chrono::{NaiveDate, Utc};
use serde::Deserialize;

pub fn configure(server: &mut ServiceConfig) {
    server
        .service(update_stock_loss)
        .service(update_stock_resupply);
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

#[post("/item/{item_id}/loss")]
async fn update_stock_loss(
    path: web::Path<i32>,
    body: web::Json<UpdateStockBody>,
    item_service: web::Data<ItemService>,
) -> impl Responder {
    let item_id = path.into_inner();
    let quantity = body.quantity;
    let date = match &body.date {
        Some(date) => NaiveDate::from_ymd_opt(date.year, date.month, date.day),
        None => Some(Utc::now().naive_utc().date()),
    };

    if date.is_none() {
        return HttpResponse::BadRequest().json("Invalid date provided");
    }

    match item_service
        .update_stock_loss(item_id, date.unwrap(), quantity)
        .await
    {
        Ok(new_quantity) => HttpResponse::Ok().json(new_quantity),
        Err(e) => match e {
            ServiceError::Input(msg) => {
                HttpResponse::BadRequest().json("Error: ".to_owned() + &msg)
            }
            _ => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        },
    }
}

#[post("/item/{item_id}/resupply")]
async fn update_stock_resupply(
    path: web::Path<i32>,
    body: web::Json<UpdateStockBody>,
    item_service: web::Data<ItemService>,
) -> impl Responder {
    let item_id = path.into_inner();
    let quantity = body.quantity;
    let date = match &body.date {
        Some(date) => NaiveDate::from_ymd_opt(date.year, date.month, date.day),
        None => Some(Utc::now().naive_utc().date()),
    };

    if date.is_none() {
        return HttpResponse::BadRequest().json("Invalid date provided");
    }

    match item_service
        .update_stock_resupply(item_id, date.unwrap(), quantity)
        .await
    {
        Ok(new_quantity) => HttpResponse::Ok().json(new_quantity),
        Err(e) => match e {
            ServiceError::Input(msg) => {
                HttpResponse::BadRequest().json("Error: ".to_owned() + &msg)
            }
            _ => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        },
    }
}
