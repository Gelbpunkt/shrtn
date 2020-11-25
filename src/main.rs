use actix_web::{get, http, post, web, App, HttpResponse, HttpServer};
use askama::Template;
use dashmap::DashMap;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::env;

mod db;

const INDEX: &[u8] = include_bytes!("../templates/index.html");

#[derive(Template)]
#[template(path = "create.html")]
struct CreateTemplate<'a> {
    url: &'a str,
}

#[derive(Deserialize)]
struct FormData {
    url: String,
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(INDEX)
}

#[post("/{url}")]
async fn get_url(
    cache: web::Data<DashMap<String, String>>,
    pool: web::Data<db::PgPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let name = format!("{}", *path);
    let target = match cache.get(&name) {
        None => match db::get_link(name.clone(), pool).await {
            None => return HttpResponse::NotFound().body("Not found."),
            Some(v) => {
                cache.insert(name.clone(), v.clone());
                v
            }
        },
        Some(v) => v.value().to_owned(),
    };
    HttpResponse::Found()
        .header(http::header::LOCATION, target)
        .finish()
        .into_body()
}

#[post("/")]
async fn create(
    cache: web::Data<DashMap<String, String>>,
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
    cache.insert(new_url.clone(), url.clone());
    db::create_link(new_url.clone(), url.clone(), pool).await;
    let temp = CreateTemplate {
        url: &format!("{}{}", env::var("BASE_URL").unwrap(), new_url),
    }
    .render()
    .unwrap();
    HttpResponse::Ok().content_type("text/html").body(temp)
}

#[actix_web::main]
async fn main() {
    let data: web::Data<DashMap<String, String>> = web::Data::new(DashMap::new());
    let pool = db::init_pool(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .data(pool.clone())
            .service(index)
            .service(get_url)
            .service(create)
    })
    .bind("0.0.0.0:4445")
    .unwrap()
    .run()
    .await
    .unwrap();
}
