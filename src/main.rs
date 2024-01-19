use actix_cors::Cors;
use actix_web::{get, http, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;

mod collections;
use collections::items::all_items;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

struct AppData {
    mdbclient: mongodb::Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok(); // Load .env

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();

    HttpServer::new(move || {
        let _cors = Cors::default()
            .allowed_origin("http://thetriangle.org")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE);

        let mdbclient = Client::with_options(options.clone()).unwrap();

        App::new()
            .app_data(web::Data::new(AppData { mdbclient }))
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(hello)
            .service(all_items)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
