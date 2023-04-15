use diesel::{r2d2::ConnectionManager, PgConnection};

pub mod handlers;
pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
