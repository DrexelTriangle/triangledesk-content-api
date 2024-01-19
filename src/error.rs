use actix_web::ResponseError;

#[derive(Debug)]
pub enum CAPIError {
    MongoDB(mongodb::error::Error),
}

impl From<mongodb::error::Error> for CAPIError {
    fn from(value: mongodb::error::Error) -> Self {
        CAPIError::MongoDB(value)
    }
}

impl ResponseError for CAPIError {}

impl std::fmt::Display for CAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CAPIError::MongoDB(e) => format!("{}", e),
        };
        f.write_str(&s)
    }
}
