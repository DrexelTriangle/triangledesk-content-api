use crate::{error::CAPIError, newsitem::NewsItem, AppData};
use actix_web::{get, web, HttpResponse, Responder};
use bson::doc;
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use utoipa::{IntoParams, Modify, ToSchema};

#[derive(Deserialize, IntoParams)]
struct ODataParams {
    #[serde(rename = "$orderby")]
    orderby: Option<String>,
}

#[utoipa::path(
    context_path="/content",
    responses(
        (status=200, description="get all news items", body=String)
    ),
    params(ODataParams)
)]
#[get("/items")] // NOT a complete OData implementation, just implementing what I need in an OData way
async fn all_items(
    data: web::Data<AppData>,
    params: web::Query<ODataParams>,
) -> Result<impl Responder, CAPIError> {
    let client = &data.mdbclient;

    let collection: mongodb::Collection<NewsItem> = client.database("content").collection("items");
    let mut results: Vec<NewsItem> = collection
        .find(None, None)
        .await?
        .map(|item| item.unwrap())
        .collect()
        .await;

    params.into_inner().orderby.map(|v| {
        let mut parts = v.split(" ");
        let field = parts.next().unwrap();

        results.sort_by(|a, b| match field {
            "versioncreated" => a.get_versioncreated().cmp(&b.get_versioncreated()),
            _ => Ordering::Equal,
        });

        if let Some("desc") = parts.next() {
            results.reverse();
        }
    });

    Ok(HttpResponse::Ok().json(results))
    // HttpResponse::Ok().body(
    // "[{\"service\": [{\"code\": \"comic\", \"name\": \"Comic\"}], \"copyrightholder\": \"The Triangle\", \"priority\": 6, \"headline\": \"The Final Grade\", \"source\": \".\", \"type\": \"text\", \"byline\": \"Matthew Lacy\", \"language\": \"en\", \"firstcreated\": \"2023-12-05T23:53:09+0000\", \"associations\": {\"featuremedia\": {\"version\": \"2\", \"body_text\": \".\", \"description_text\": \".\", \"service\": [{\"code\": \"comic\", \"name\": \"Comic\"}], \"copyrightholder\": \"The Triangle\", \"guid\": \"tag:desk.thetriangle.org:2023:e9c8d5fd-434c-4fbc-86cc-dc973525ae7a\", \"pubstatus\": \"usable\", \"headline\": \".\", \"mimetype\": \"image/png\", \"type\": \"picture\", \"source\": \"Superdesk\", \"usageterms\": \"\", \"copyrightnotice\": \"Copyright 2019 The Triangle. All rights reserved.\", \"language\": \"en\", \"versioncreated\": \"2023-12-05T23:53:42+0000\", \"firstcreated\": \"2023-12-05T23:53:41+0000\", \"urgency\": 3, \"priority\": 6, \"renditions\": {\"original\": {\"poi\": {\"x\": 587, \"y\": 831}, \"href\": \"https://desk.thetriangle.org/api/upload-raw/656fb8053b77501e7dfad1c8.png\", \"width\": 1174, \"mimetype\": \"image/png\", \"height\": 1662, \"media\": \"656fb8053b77501e7dfad1c8\"}}, \"genre\": [{\"code\": \"Article\", \"name\": \"Article (news)\"}]}}, \"profile\": \"Comic\", \"version\": \"7\", \"usageterms\": \"\", \"firstpublished\": \"2023-12-08T00:07:29+0000\", \"genre\": [{\"code\": \"Article\", \"name\": \"Article (news)\"}], \"body_html\": \"\", \"pubstatus\": \"usable\", \"guid\": \"urn:newsml:desk.thetriangle.org:2023-12-05T18:53:09.197441:0bad136b-6288-4b9f-b0e0-afac1f4e3cec\", \"readtime\": 0, \"versioncreated\": \"2023-12-05T23:54:07+0000\", \"wordcount\": 0, \"urgency\": 3, \"charcount\": 0, \"copyrightnotice\": \"Copyright 2019 The Triangle. All rights reserved.\"}]")
}

#[utoipa::path( context_path="/content",
    responses(
        (status=200, description="get news item by id", body=String)
    )
)]
#[get("/items/{_id}")]
async fn item_by_id(
    data: web::Data<AppData>,
    id: web::Path<String>,
) -> Result<impl Responder, CAPIError> {
    let client = &data.mdbclient;

    let collection: mongodb::Collection<NewsItem> = client.database("content").collection("items");
    let item = collection
        .find_one(doc! {"_id": id.into_inner()}, None)
        .await?;

    match item {
        Some(d) => Ok(HttpResponse::Ok().json(d)),
        None => Ok(HttpResponse::NotFound().into()),
    }
}
