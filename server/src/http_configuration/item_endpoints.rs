use crate::dtos::item_dtos::{NewItemRequest, UpdateItemRequest};
use crate::services::error::ServiceError;
use crate::services::item_service::ItemService;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::{delete, get, post, put, web, Responder};

pub fn configure(server: &mut ServiceConfig) {
    server
        .service(get_items)
        .service(get_item)
        .service(create_item)
        .service(update_item)
        .service(remove_item);
}

#[get("/item")]
async fn get_items(item_service: web::Data<ItemService>) -> impl Responder {
    match item_service.get_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

#[get("/item/{item_id}")]
async fn get_item(path: web::Path<i32>, item_service: web::Data<ItemService>) -> impl Responder {
    let item_id = path.into_inner();

    match item_service.get_item(item_id).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => match e {
            ServiceError::Database(database_error) => match database_error.0 {
                crate::database::error::Kind::ItemNotFound => {
                    HttpResponse::NotFound().json("Item not found")
                }
                _ => HttpResponse::InternalServerError().json(format!("Error: {}", database_error)),
            },
            _ => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        },
    }
}

#[post("/item")]
async fn create_item(
    body: web::Json<NewItemRequest>,
    item_service: web::Data<ItemService>,
) -> impl Responder {
    match item_service.create_item(body.into_inner()).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => match e {
            ServiceError::Database(database_error) => match database_error.0 {
                crate::database::error::Kind::ItemNotFound => {
                    HttpResponse::NotFound().json("Item not found")
                }
                _ => HttpResponse::InternalServerError().json(format!("Error: {}", database_error)),
            },
            _ => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        },
    }
}

#[put("/item/{item_id}")]
async fn update_item(
    path: web::Path<i32>,
    body: web::Json<UpdateItemRequest>,
    item_service: web::Data<ItemService>,
) -> impl Responder {
    let item_id = path.into_inner();

    if item_id != body.id {
        return HttpResponse::BadRequest().json("Item id in path and body do not match");
    }

    match item_service.update_item(body.into_inner()).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => match e {
            ServiceError::Database(database_error) => match database_error.0 {
                crate::database::error::Kind::ItemNotFound => {
                    HttpResponse::NotFound().json("Item not found")
                }
                _ => HttpResponse::InternalServerError().json(format!("Error: {}", database_error)),
            },
            _ => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        },
    }
}

#[delete("/item/{item_id}")]
async fn remove_item(path: web::Path<i32>, item_service: web::Data<ItemService>) -> impl Responder {
    let item_id = path.into_inner();

    match item_service.remove_item(item_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => match e {
            ServiceError::Database(database_error) => match database_error.0 {
                crate::database::error::Kind::ItemNotFound => {
                    HttpResponse::NotFound().json("Item not found")
                }
                _ => HttpResponse::InternalServerError().json(format!("Error: {}", database_error)),
            },
            _ => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
        },
    }
}
