use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Index {}

#[get("/")]
async fn index_redirect() -> impl Responder {
    HttpResponse::Found().append_header(("Location", "/homepage")).finish()
}

#[get("/homepage")]
async fn homepage() -> impl Responder {
    HttpResponse::Ok().body(Index {}.render_once().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(index_redirect)
            .service(homepage)
    }).bind(("127.0.0.1", 80))?.run().await
}