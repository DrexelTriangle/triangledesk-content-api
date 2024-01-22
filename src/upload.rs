use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

use crate::newsitem::NewsItem;
use crate::{error::CAPIError, AppData};
use actix_web::{post, web, HttpResponse, Responder};

#[utoipa::path(
    responses(
        (status=200, description="get news item by id", body=String)
    )
)]
#[post("/upload")]
async fn upload_item(
    data: web::Data<AppData>,
    item: web::Json<NewsItem>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, CAPIError> {
    println!("{:?}", item.into_inner());
    let peer_ip = req.peer_addr().unwrap().ip();
    let var = env::var("UPLOADER_IPS").unwrap_or(String::new());
    let mut allowed_ips = var
        .split(",")
        .into_iter()
        .filter_map(|ipstr| Some(IpAddr::V4(Ipv4Addr::from_str(ipstr).ok()?)));
    println!("{:?}", peer_ip);
    if allowed_ips.find(|ip| *ip == peer_ip).is_some() {
        println!("this ip is allowed")
    } else {
        log::warn!(
            "UNAUTHORIZED IP {} BLOCKED FROM ACCESSING UPLOAD ENDPOINT",
            peer_ip
        )
    }

    Ok(HttpResponse::Ok())
}
