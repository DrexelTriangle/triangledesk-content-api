use crate::AppData;
use actix_web::{get, web, HttpResponse, Responder};
use bson::Document;
use futures_util::StreamExt;
use mongodb::Cursor;

#[get("/items")]
async fn all_items(data: web::Data<AppData>) -> impl Responder {
    let client = &data.mdclient;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await.unwrap() {
        println!("- {}", name);
    }

    let collection: mongodb::Collection<Document> = client.database("content").collection("items");
    let mut allItems: Cursor<Document> = collection.find(None, None).await.unwrap();
    let itemsVec: Vec<Document> = allItems.map(|item| item.unwrap()).collect().await;

    let mut resp = itemsVec.into_iter().fold(String::from("["), |acc, doc| {
        acc + &format!("{}", doc) + ","
    });
    resp.pop();
    resp += "]";

    HttpResponse::Ok().body(resp)
    // HttpResponse::Ok().body(
    // "[{\"service\": [{\"code\": \"comic\", \"name\": \"Comic\"}], \"copyrightholder\": \"The Triangle\", \"priority\": 6, \"headline\": \"The Final Grade\", \"source\": \".\", \"type\": \"text\", \"byline\": \"Matthew Lacy\", \"language\": \"en\", \"firstcreated\": \"2023-12-05T23:53:09+0000\", \"associations\": {\"featuremedia\": {\"version\": \"2\", \"body_text\": \".\", \"description_text\": \".\", \"service\": [{\"code\": \"comic\", \"name\": \"Comic\"}], \"copyrightholder\": \"The Triangle\", \"guid\": \"tag:desk.thetriangle.org:2023:e9c8d5fd-434c-4fbc-86cc-dc973525ae7a\", \"pubstatus\": \"usable\", \"headline\": \".\", \"mimetype\": \"image/png\", \"type\": \"picture\", \"source\": \"Superdesk\", \"usageterms\": \"\", \"copyrightnotice\": \"Copyright 2019 The Triangle. All rights reserved.\", \"language\": \"en\", \"versioncreated\": \"2023-12-05T23:53:42+0000\", \"firstcreated\": \"2023-12-05T23:53:41+0000\", \"urgency\": 3, \"priority\": 6, \"renditions\": {\"original\": {\"poi\": {\"x\": 587, \"y\": 831}, \"href\": \"https://desk.thetriangle.org/api/upload-raw/656fb8053b77501e7dfad1c8.png\", \"width\": 1174, \"mimetype\": \"image/png\", \"height\": 1662, \"media\": \"656fb8053b77501e7dfad1c8\"}}, \"genre\": [{\"code\": \"Article\", \"name\": \"Article (news)\"}]}}, \"profile\": \"Comic\", \"version\": \"7\", \"usageterms\": \"\", \"firstpublished\": \"2023-12-08T00:07:29+0000\", \"genre\": [{\"code\": \"Article\", \"name\": \"Article (news)\"}], \"body_html\": \"\", \"pubstatus\": \"usable\", \"guid\": \"urn:newsml:desk.thetriangle.org:2023-12-05T18:53:09.197441:0bad136b-6288-4b9f-b0e0-afac1f4e3cec\", \"readtime\": 0, \"versioncreated\": \"2023-12-05T23:54:07+0000\", \"wordcount\": 0, \"urgency\": 3, \"charcount\": 0, \"copyrightnotice\": \"Copyright 2019 The Triangle. All rights reserved.\"}]")
}
