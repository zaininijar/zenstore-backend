use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}