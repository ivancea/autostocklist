use crate::dtos::forecast_dtos::ForecastQuery;
use crate::services::error::ServiceError;
use crate::services::forecast_service::{ForecastConfig, ForecastService};
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::{get, web, Responder};

pub fn configure(server: &mut ServiceConfig) {
    server.service(get_item_forecast).service(get_forecast_summary);
}

#[get("/item/{item_id}/forecast")]
async fn get_item_forecast(
    path: web::Path<i32>,
    query: web::Query<ForecastQuery>,
    forecast_service: web::Data<ForecastService>,
) -> impl Responder {
    let item_id = path.into_inner();
    let config = match ForecastConfig::from_query(query.history_days, query.forecast_days) {
        Ok(config) => config,
        Err(e) => return handle_service_error(e),
    };

    match forecast_service.forecast_item(item_id, config).await {
        Ok(forecast) => HttpResponse::Ok().json(forecast),
        Err(e) => handle_service_error(e),
    }
}

#[get("/forecast")]
async fn get_forecast_summary(
    query: web::Query<ForecastQuery>,
    forecast_service: web::Data<ForecastService>,
) -> impl Responder {
    let config = match ForecastConfig::from_query(query.history_days, query.forecast_days) {
        Ok(config) => config,
        Err(e) => return handle_service_error(e),
    };

    match forecast_service.forecast_all(config).await {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(e) => handle_service_error(e),
    }
}

fn handle_service_error(error: ServiceError) -> HttpResponse {
    match error {
        ServiceError::Input(msg) => HttpResponse::BadRequest().json("Error: ".to_owned() + &msg),
        ServiceError::Database(database_error) => match database_error.0 {
            crate::database::error::Kind::ItemNotFound => {
                HttpResponse::NotFound().json("Item not found")
            }
            _ => HttpResponse::InternalServerError().json(format!("Error: {}", database_error)),
        },
    }
}
