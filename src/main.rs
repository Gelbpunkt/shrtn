#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{http, web, App, Error, HttpResponse, HttpServer};
use askama::Template;
use bytes::Bytes;
use futures::stream::once;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

mod db;
mod model;
mod schema;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "create.html")]
struct CreateTemplate<'a> {
    url: &'a str,
}

#[derive(Deserialize)]
struct FormData {
    url: String,
}

fn index() -> HttpResponse {
    let temp = Bytes::from(IndexTemplate.render().unwrap().as_bytes());
    let body = once::<Bytes, Error>(Ok(temp));
    HttpResponse::Ok()
        .content_type("text/html")
        .streaming(Box::new(body))
}

fn get(
    cache: web::Data<Mutex<HashMap<String, String>>>,
    pool: web::Data<db::PgPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let name = format!("{}", *path);
    let target;
    match cache.lock().unwrap().get(&name) {
        None => match db::get_link(name.clone(), &pool) {
            Err(_) => return HttpResponse::NotFound().body("Not found."),
            Ok(v) => {
                target = v.to;
                cache.lock().unwrap().insert(name.clone(), target.clone());
            }
        },
        Some(v) => target = v.to_string(),
    }
    HttpResponse::Found()
        .header(http::header::LOCATION, target)
        .finish()
        .into_body()
}

fn create(
    cache: web::Data<Mutex<HashMap<String, String>>>,
    pool: web::Data<db::PgPool>,
    form: web::Form<FormData>,
) -> HttpResponse {
    let url = (&form.url).to_string();
    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
    let new_url;
    if url.matches("/").count() > 2 && url.split("/").last().unwrap().matches(".").count() > 0 {
        let ext = url.split(".").last().unwrap();
        new_url = format!("{}.{}", rand_string, ext);
    } else {
        new_url = format!("{}", rand_string);
    }
    cache.lock().unwrap().insert(new_url.clone(), url.clone());
    db::create_link(new_url.clone(), url.clone(), &pool).unwrap();
    let temp = Bytes::from(
        CreateTemplate {
            url: &format!("{}{}", env::var("BASE_URL").unwrap(), new_url),
        }
        .render()
        .unwrap()
        .as_bytes(),
    );
    let body = once::<Bytes, Error>(Ok(temp));
    HttpResponse::Ok()
        .content_type("text/html")
        .streaming(Box::new(body))
}

fn main() {
    let pool = web::Data::new(
        db::init_pool(&env::var("DATABASE_URL").unwrap()).expect("Failed to create pool"),
    );
    let data: web::Data<Mutex<HashMap<String, String>>> =
        web::Data::new(Mutex::new(HashMap::new()));
    println!("Initializing shrtn to run on port 4445");
    HttpServer::new(move || {
        App::new()
            .register_data(data.clone())
            .register_data(pool.clone())
            .route("/", web::get().to_async(index))
            .route("/", web::post().to_async(create))
            .route("/{url}", web::get().to_async(get))
    })
    .bind("0.0.0.0:4445")
    .unwrap()
    .run()
    .unwrap();
}
