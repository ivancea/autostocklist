use crate::database::{error::Kind, Database};
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::{get, web, Responder};

pub fn configure(server: &mut ServiceConfig) {
    server.service(web::scope("/item").service(get_items).service(get_item));
}

#[get("")]
async fn get_items(database: web::Data<Database>) -> impl Responder {
    match database.get_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[get("/{item_id}")]
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
