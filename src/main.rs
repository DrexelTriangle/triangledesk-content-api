mod error;
mod items;
mod newsitem;
mod upload;

use actix_cors::Cors;
use actix_web::{get, http, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use newsitem::{NewsItem, NewsItemSchema, NewsService};
use std::{env, error::Error};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

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

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    match dotenv::dotenv() {
        Ok(_) => log::info!(".env file loaded"),
        Err(_) => log::warn!(
            ".env file not found; make sure you've declared all environment variables you need"
        ),
    }

    let client_uri = // Ok to panic because it's developer error not user error
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;

    #[derive(OpenApi)]
    #[openapi(
        paths(hello, items::all_items, items::item_by_id, upload::upload_item),
        components(schemas(NewsItem, NewsService)),
        tags(
            (name="items", description="Published news items"),
            (name="upload", description="Push new items. Restricted to certain IPs")
        ),
        modifiers(&NewsItemSchema)
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    Ok(HttpServer::new(move || {
        let _cors = Cors::default()
            .allowed_origin("http://thetriangle.org")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE);

        // Have to unwrap because it's inside closure
        let mdbclient = Client::with_options(options.clone()).unwrap();

        let base_url = env::var("BASE_URL").unwrap_or(String::from(""));

        App::new()
            .app_data(web::Data::new(AppData { mdbclient }))
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(hello)
            .service(items::all_items)
            .service(items::item_by_id)
            .service(upload::upload_item)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
                    .config(Config::new([base_url + "/api-docs/openapi.json"])),
            )
    })
    .bind(("0.0.0.0", 52892))?
    .run()
    .await?)
}
