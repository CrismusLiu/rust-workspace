use actix_web::{web, App, HttpServer, Responder, HttpResponse};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(hanlders));
}

pub async fn hanlders() -> impl Responder {
    HttpResponse::Ok().json("Actix web service is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let app = move || App::new().configure(routes);

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await

}
