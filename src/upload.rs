use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

use crate::newsitem::NewsItem;
use crate::{error::CAPIError, AppData};
use actix_web::http::header::X_FORWARDED_FOR;
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
    let peer_ip: Ipv4Addr = match req.peer_addr().unwrap().ip() {
        IpAddr::V4(v) => v,
        IpAddr::V6(_) => todo!(),
    };
    let real_ip = req
        .headers()
        .get(X_FORWARDED_FOR)
        .and_then(|val| {
            env::var("TRUSTED_PROXIES")
                .map(|proxy_var| {
                    let proxies = proxy_var.split(",").into_iter().filter_map(|ipstr| {
                        if let Ok(ipv4) = Ipv4Addr::from_str(ipstr.trim()) {
                            Some(ipnet::Ipv4Net::new(ipv4, 1).unwrap())
                        } else {
                            ipnet::Ipv4Net::from_str(ipstr.trim()).ok()
                        }
                    });
                    get_real_ip(peer_ip, val.to_str().unwrap(), proxies.collect())
                })
                .ok()
        })
        .unwrap_or(peer_ip);

    let uploader_var = env::var("UPLOADER_IPS").unwrap_or(String::new());
    let mut allowed_ips = uploader_var
        .split(",")
        .into_iter()
        .filter_map(|ipstr| Some(IpAddr::V4(Ipv4Addr::from_str(ipstr.trim()).ok()?)));

    if allowed_ips.find(|ip| *ip == real_ip).is_some() {
        println!("this ip is allowed");
        Ok(HttpResponse::Ok())
    } else {
        log::warn!(
            "UNAUTHORIZED IP {} BLOCKED FROM ACCESSING UPLOAD ENDPOINT",
            real_ip
        );
        Ok(HttpResponse::Forbidden())
    }
}

fn get_real_ip(
    peer_ip: Ipv4Addr,
    x_forwarded_for: &str,
    proxies: iprange::IpRange<ipnet::Ipv4Net>,
) -> Ipv4Addr {
    let mut last_peer = peer_ip;
    let mut forwards: Vec<&str> = x_forwarded_for.split(",").collect();
    while proxies.contains(&last_peer) && forwards.len() > 0 {
        let fwd = forwards.pop().unwrap();
        last_peer = Ipv4Addr::from_str(fwd.trim()).unwrap();
    }
    last_peer
}
