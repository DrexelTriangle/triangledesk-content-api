use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::{openapi::OpenApi, Modify, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewsService {
    code: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct MediaRendition {
    href: String,
    width: i32,
    height: i32,
    media: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct NewsItem {
    #[serde(rename = "_id")]
    id: Option<String>,
    guid: String,
    #[serde(rename = "type")]
    itemtype: String,
    language: String,
    readtime: Option<i32>,
    wordcount: Option<i32>,
    charcount: Option<i32>,

    copyrightholder: String,
    copyrightnotice: String,

    slugline: Option<String>,
    headline: String,
    byline: Option<String>,
    service: Vec<NewsService>,
    body_html: Option<String>,
    versioncreated: DateTime<Utc>,
    firstpublished: Option<DateTime<Utc>>,

    associations: Option<HashMap<String, NewsItem>>,
    renditions: Option<HashMap<String, MediaRendition>>,
}

impl NewsItem {
    pub fn get_id(&self) -> String {
        self.id.clone().unwrap_or(self.guid.clone())
    }
    pub fn get_versioncreated(&self) -> DateTime<Utc> {
        self.versioncreated
    }
}

pub struct NewsItemSchema;

impl Modify for NewsItemSchema {
    fn modify(&self, openapi: &mut OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        let schema = components.schemas.get_mut("NewsItem").unwrap();

        match schema {
            utoipa::openapi::RefOr::Ref(_) => todo!(), // tf is this type idk how to extract from it
            utoipa::openapi::RefOr::T(s) => match s {
                utoipa::openapi::Schema::Object(obj) => {
                    obj.properties.insert(
                        "versioncreated".to_owned(),
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(utoipa::openapi::SchemaType::String)
                            .into(),
                    );
                    obj.properties.insert(
                        "firstpublished".to_owned(),
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(utoipa::openapi::SchemaType::String)
                            .into(),
                    )
                }
                _ => unreachable!(),
            },
        };
    }
}
