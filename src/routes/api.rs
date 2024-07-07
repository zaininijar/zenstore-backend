use actix_web::web;
use crate::handlers::product::{get_products, create_product, get_product, update_product, delete_product};
use crate::config::database::Pool;

pub fn init(cfg: &mut web::ServiceConfig, pool: Pool) {
    let pool = web::Data::new(pool);
    cfg
        .app_data(pool.clone())
        .service(
            web::resource("/products")
                .route(web::get().to(get_products))
                .route(web::post().to(create_product))
        )
        .service(
            web::resource("/products/{id}")
                .route(web::get().to(get_product))
                .route(web::put().to(update_product))
                .route(web::delete().to(delete_product))
        );
}
