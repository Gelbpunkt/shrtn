use actix::{Actor, Addr};
use actix_web::{
    get, http::header::LOCATION, middleware::Logger, post, web, App, HttpResponse, HttpServer,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use std::env;

mod redis_actor;

const INDEX: &[u8] = include_bytes!("../templates/index.html");
const CREATE: &str = include_str!("../templates/create.html");

#[derive(Deserialize)]
struct FormData {
    url: String,
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(INDEX)
}

#[get("/{url}")]
async fn get_url(
    db: web::Data<Addr<redis_actor::RedisActor>>,
    path: web::Path<String>,
) -> HttpResponse {
    let redis_key = format!("shrtn:{}", path);
    let result = db
        .send(redis_actor::GetCommand(redis_key))
        .await
        .unwrap()
        .unwrap();

    if let Some(url) = result {
        HttpResponse::Found()
            .append_header((LOCATION, url))
            .finish()
            .into_body()
    } else {
        HttpResponse::NotFound().body("Not found.")
    }
}

#[post("/")]
async fn create(
    db: web::Data<Addr<redis_actor::RedisActor>>,
    form: web::Form<FormData>,
) -> HttpResponse {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let url = &form.url;
    let new_url = {
        if url.matches("/").count() > 2 && url.split("/").last().unwrap().matches(".").count() > 0 {
            let ext = url.split(".").last().unwrap();
            format!("{}.{}", rand_string, ext)
        } else {
            format!("{}", rand_string)
        }
    };
    let redis_key = format!("shrtn:{}", &new_url);
    db.send(redis_actor::SetCommand(redis_key, url.clone()))
        .await
        .unwrap()
        .unwrap();
    let text = CREATE.replace(
        "URL_GOES_HERE",
        &format!("{}{}", env::var("BASE_URL").unwrap(), new_url),
    );
    HttpResponse::Ok().content_type("text/html").body(text)
}

#[actix_web::main]
async fn main() {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let actor = redis_actor::RedisActor::new(&env::var("DATABASE_URL").unwrap()).await;
    let addr = actor.start();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(addr.clone())
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
