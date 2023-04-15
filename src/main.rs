use std::env;

use actix_files::Files;
use actix_web::middleware;
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use handlebars::Handlebars;
use rust_cookie_webapp::handlers::*;

use rust_cookie_webapp::{handlers::*, DbPool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebar_data = web::Data::new(handlebars);

    // Database setup
    let database_url = env::var("DATABASE_URL").expect("ERROR: DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("ERROR: Failed to create DB connection pool");

    println!("Serving on Port 8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(handlebar_data.clone())
            .app_data(web::Data::new(pool.clone()))
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
            .service(add)
            .service(add_new_form)
            .service(cookie)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
