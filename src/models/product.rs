// models.rs atau file terpisah yang diperlukan
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::products;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub price: f64,
    pub description: String,
}
