use crate::dtos::stock_dtos::UpdateStockRequest;
use crate::services::error::ServiceError;
use crate::services::stock_service::StockService;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::{put, web, Responder};
use chrono::{NaiveDate, Utc};

pub fn configure(server: &mut ServiceConfig) {
    server
        .service(update_stock_loss)
        .service(update_stock_resupply);
}

#[put("/item/{item_id}/loss")]
async fn update_stock_loss(
    path: web::Path<i32>,
    body: web::Json<UpdateStockRequest>,
    stock_service: web::Data<StockService>,
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

    match stock_service
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

#[put("/item/{item_id}/resupply")]
async fn update_stock_resupply(
    path: web::Path<i32>,
    body: web::Json<UpdateStockRequest>,
    item_service: web::Data<StockService>,
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
