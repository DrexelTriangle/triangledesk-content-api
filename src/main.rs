mod content;
mod error;

use actix_cors::Cors;
use actix_web::{get, http, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use content::items;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::{env, error::Error};
use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

#[utoipa::path(
    responses((status = 200, description = "root successful, service exists", body=String))
)]
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

struct AppData {
    mdbclient: mongodb::Client,
}

#[derive(OpenApi)]
#[openapi(paths(hello, items::all_items))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv()?; // Load .env

    let client_uri = // Ok to panic because it's developer error not user error
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;

    let openapi = ApiDoc::openapi();

    Ok(HttpServer::new(move || {
        let _cors = Cors::default()
            .allowed_origin("http://thetriangle.org")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE);

        // Have to unwrap because it's inside closure
        let mdbclient = Client::with_options(options.clone()).unwrap();

        let content = web::scope("/content").service(items::all_items);

        App::new()
            .app_data(web::Data::new(AppData { mdbclient }))
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(hello)
            .service(content)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?)
}
