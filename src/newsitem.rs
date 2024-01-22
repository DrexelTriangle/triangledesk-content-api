use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::{openapi::OpenApi, Modify, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewsService {
    code: String,
    name: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct NewsItem {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "type")]
    itemtype: String,
    language: String,
    readtime: i32,
    wordcount: i32,
    charcount: i32,

    copyrightholder: String,
    copyrightnotice: String,

    slugline: Option<String>,
    headline: String,
    byline: String,
    service: Vec<NewsService>,
    body_html: String,
    versioncreated: DateTime<Utc>,
    firstpublished: DateTime<Utc>,
}

impl NewsItem {
    pub fn get_id(&self) -> String {
        self.id.to_string()
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
