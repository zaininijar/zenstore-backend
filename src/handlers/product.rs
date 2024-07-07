use actix_web::{web, HttpResponse, Responder, Result};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{models::{Product, NewProduct}, schema::products::dsl::*};
use crate::config::database::Pool;

#[derive(Serialize)]
struct SuccessResponse<T> {
    data: T,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}


pub async fn get_products(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to get DB connection".to_string(),
        })),
    };

    let result = web::block(move || {
        products.load::<Product>(&mut conn)
    }).await;

    match result {
        Ok(products_list) => {
            let response = SuccessResponse {
                data: products_list,
                message: "Products loaded successfully".to_string(),
            };
            Ok(HttpResponse::Ok().json(response));
        }
        Err(e) => {
            // Serialize error into JSON response
            let error_response = ErrorResponse {
                error: format!("Failed to load products: {}", e),
            };
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

pub async fn create_product(pool: web::Data<Pool>, item: web::Json<NewProduct>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let new_product = item.into_inner();

    let result = web::block(move || {
        diesel::insert_into(products)
            .values(&new_product)
            .execute(&conn)
    })
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Created().json(result))
}

pub async fn get_product(pool: web::Data<Pool>, product_id: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || products.find(product_id.into_inner()).first::<Product>(&conn))
        .await
        .map_err(|_| HttpResponse::NotFound().finish())?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_product(pool: web::Data<Pool>, product_id: web::Path<i32>, item: web::Json<NewProduct>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || {
        diesel::update(products.find(product_id.into_inner()))
            .set((
                name.eq(item.name.clone()),
                price.eq(item.price),
                description.eq(item.description.clone()),
            ))
            .execute(&conn)
    })
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_product(pool: web::Data<Pool>, product_id: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || diesel::delete(products.find(product_id.into_inner())).execute(&conn))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().json(result))
}
