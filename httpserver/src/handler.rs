use std::io::Write;
use http::{httprequest, httprequest::{HttpRequest, Method}, httpresponse::HttpResponse};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env;
use std::fs;



pub trait Handler {

    fn handle(req: &HttpRequest) -> HttpResponse;

    /// 读取指定路径静态文件
    fn load_file(file_name: &str) -> Option<String> {
        let dir = env!("CARGO_MANIFEST_DIR");

        let default_path = format!("{}/public", dir);
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}


pub struct WebServiceHandler;
pub struct StaticPageHandler;
pub struct PageNotFoundHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

impl Handler for PageNotFoundHandler {

    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        match &req.resource {
            httprequest::Resource::Path(s) => {
                let routes: Vec<&str> = s.split("/").collect();

                let acture_url = routes[1];

                match acture_url {
                    "" => HttpResponse::new("200", None, Self::load_file("index.html")),
                    "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
                    path => match Self::load_file(path) {
                        Some(content) => {
                            let mut headers: HashMap<&str, &str> = HashMap::new();

                            if path.ends_with(".css") {
                                headers.insert("Content-Type", "text/css");
                            } else if path.ends_with(".js") {
                                headers.insert("Content-Type", "text/javascript");                                
                            } else {
                                headers.insert("Content-Type", "text/html");
                            }

                            HttpResponse::new("200", Some(headers), Some(content))
                        },
                        None => HttpResponse::new("404", None, Self::load_file("404.html")),
                    }
                }
            }
        }
    }
}


impl WebServiceHandler {

    pub fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);

        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);

        let orders: Vec<OrderStatus> = serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }


}

impl Handler for WebServiceHandler {
    
    fn handle(req: &HttpRequest) -> HttpResponse {
        let httprequest::Resource::Path(url) = &req.resource;

        let routes: Vec<&str> = url.split("/").collect();

        match routes[2] {
            "shopping" if routes.len() > 2 && routes[3] == "order" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());

                let mut headers = HashMap::new();
                headers.insert("Content-Type", "application/json");

                HttpResponse::new("200", Some(headers), body)
            },
            _ => HttpResponse::new("400", None, Self::load_file("404.html"))
        }
    }

}




















