use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{openapi::OpenApi, Modify, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewsService {
    code: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct NewsItem {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "type")]
    itemtype: String,
    language: String,

    copyrightholder: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    slugline: Option<String>,
    headline: String,
    byline: String,
    service: Vec<NewsService>,
    body_html: String,
    versioncreated: DateTime<Utc>,
}

impl NewsItem {
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
                utoipa::openapi::Schema::Object(obj) => obj.properties.insert(
                    "versioncreated".to_owned(),
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::String)
                        .into(),
                ),
                _ => unreachable!(),
            },
        };
    }
}
