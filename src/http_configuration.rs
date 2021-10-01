mod item_endpoints;
mod stock_endpoints;

use actix_web::web::ServiceConfig;

pub fn configure(server: &mut ServiceConfig) {
    item_endpoints::configure(server);
    stock_endpoints::configure(server);
}
