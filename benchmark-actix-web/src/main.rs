use actix_web::web::get;
use actix_web::{App, HttpServer, Responder};

/// Actix-Web
/// Commands: wrk -t10 -c100 -d10s http://127.0.0.1:8080/index
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/index", get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
async fn index() -> impl Responder {
    "Hello, Actix!"
}
