use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = env::var("ADDRESS").unwrap();
    println!("start port {}", bind_address);
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin_fn(|_origin, _req_head| true)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new().wrap(cors).service(index)
    })
    .bind(bind_address)?
    .run()
    .await
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"data":{"word":"Hello my name is rust_back2","name":"rust_back2"}}"#)
}
