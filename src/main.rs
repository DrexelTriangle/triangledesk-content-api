use actix_cors::Cors;
use actix_web::{
    get, http, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/items")]
async fn items() -> impl Responder {
    HttpResponse::Ok().body(
    "[{\"service\": [{\"code\": \"comic\", \"name\": \"Comic\"}], \"copyrightholder\": \"The Triangle\", \"priority\": 6, \"headline\": \"The Final Grade\", \"source\": \".\", \"type\": \"text\", \"byline\": \"Matthew Lacy\", \"language\": \"en\", \"firstcreated\": \"2023-12-05T23:53:09+0000\", \"associations\": {\"featuremedia\": {\"version\": \"2\", \"body_text\": \".\", \"description_text\": \".\", \"service\": [{\"code\": \"comic\", \"name\": \"Comic\"}], \"copyrightholder\": \"The Triangle\", \"guid\": \"tag:desk.thetriangle.org:2023:e9c8d5fd-434c-4fbc-86cc-dc973525ae7a\", \"pubstatus\": \"usable\", \"headline\": \".\", \"mimetype\": \"image/png\", \"type\": \"picture\", \"source\": \"Superdesk\", \"usageterms\": \"\", \"copyrightnotice\": \"Copyright 2019 The Triangle. All rights reserved.\", \"language\": \"en\", \"versioncreated\": \"2023-12-05T23:53:42+0000\", \"firstcreated\": \"2023-12-05T23:53:41+0000\", \"urgency\": 3, \"priority\": 6, \"renditions\": {\"original\": {\"poi\": {\"x\": 587, \"y\": 831}, \"href\": \"https://desk.thetriangle.org/api/upload-raw/656fb8053b77501e7dfad1c8.png\", \"width\": 1174, \"mimetype\": \"image/png\", \"height\": 1662, \"media\": \"656fb8053b77501e7dfad1c8\"}}, \"genre\": [{\"code\": \"Article\", \"name\": \"Article (news)\"}]}}, \"profile\": \"Comic\", \"version\": \"7\", \"usageterms\": \"\", \"firstpublished\": \"2023-12-08T00:07:29+0000\", \"genre\": [{\"code\": \"Article\", \"name\": \"Article (news)\"}], \"body_html\": \"\", \"pubstatus\": \"usable\", \"guid\": \"urn:newsml:desk.thetriangle.org:2023-12-05T18:53:09.197441:0bad136b-6288-4b9f-b0e0-afac1f4e3cec\", \"readtime\": 0, \"versioncreated\": \"2023-12-05T23:54:07+0000\", \"wordcount\": 0, \"urgency\": 3, \"charcount\": 0, \"copyrightnotice\": \"Copyright 2019 The Triangle. All rights reserved.\"}]")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        let _cors = Cors::default()
            .allowed_origin("http://thetriangle.org")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE);

        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(hello)
            .service(items)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
