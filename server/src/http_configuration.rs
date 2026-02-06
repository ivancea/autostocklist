mod forecast_endpoints;
mod item_endpoints;
mod stock_endpoints;

use actix_web::web::ServiceConfig;

pub fn configure(server: &mut ServiceConfig) {
    forecast_endpoints::configure(server);
    item_endpoints::configure(server);
    stock_endpoints::configure(server);
}
