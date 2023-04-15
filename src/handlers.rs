use std::collections::HashMap;

use crate::models::{Cookie, NewCookie};
use crate::DbPool;
use actix_web::error::ErrorInternalServerError;
use actix_web::http::header;
use actix_web::{get, post, web, Error, HttpResponse};
use awmp::Parts;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct IndexTemplateData {
    project_name: String,
    cookies: Vec<Cookie>,
}

pub async fn index(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    use crate::schema::cookies::dsl::*;
    let cookies_data = web::block(move || {
        let mut conn = pool.get().expect("Unable to get connection from pool");
        cookies.limit(100).load::<Cookie>(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    let data = IndexTemplateData {
        project_name: "Cookies".into(),
        cookies: cookies_data,
    };

    let body = hb.render("index", &data).unwrap();
    log::warn!("{body:?}");

    Ok(HttpResponse::Ok().body(body))
}

#[get("/add")]
pub async fn add(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, Error> {
    let body = hb.render("add", &{}).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

#[post("/add_cookie_form")]
pub async fn add_new_form(
    pool: web::Data<DbPool>,
    mut parts: Parts,
) -> Result<HttpResponse, Error> {
    use crate::schema::cookies::dsl::*;
    let file_path = parts
        .files
        .take("image")
        .pop()
        .and_then(|f| f.persist_in("./static/image").ok())
        .unwrap_or_default();

    let text_fields: HashMap<_, _> = parts.texts.as_pairs().into_iter().collect();
    let mut conn = pool.get().expect("ERROR: Cannot get connection from pool");

    let new_cookie = NewCookie {
        name: text_fields.get("name").unwrap().to_string(),
        image_path: file_path.to_string_lossy().to_string(),
    };

    web::block(move || {
        diesel::insert_into(cookies)
            .values(&new_cookie)
            .execute(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

#[get("/cookie/{cookie_id}")]
pub async fn cookie(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<DbPool>,
    cookie_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    use crate::schema::cookies::dsl::*;
    let result = web::block(move || {
        let mut conn = pool.get().expect("ERROR: Cannot get connection from pool");
        cookies
            .filter(id.eq(cookie_id.into_inner()))
            .first::<Cookie>(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    log::info!("Cookie: {result:?}");

    let body = hb
        .render("cookie", &result)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(body))
}
