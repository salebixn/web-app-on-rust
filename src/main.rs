use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use sailfish::TemplateOnce;
extern crate chrono;
use chrono::{DateTime, Utc};

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Index {}

fn with_pointer(pointer: *mut u128) {
    unsafe {
        loop {
            if *pointer == 500000000 {
                break;
            }
            *pointer = *pointer + 1;
        }
    }
}

/*fn with_link(mut link: &u128) {
    loop {
        if link == &500000000 {
            break;
        }
        link = link + 1;
    }
}*/

fn without_pointer() -> u128 {
    let mut num: u128 = 1;
    loop {
        if num == 500000000 {
            break;
        }
        num = num + 1;
    }

    return num;
}

#[get("/")]
async fn index_redirect() -> impl Responder {
    HttpResponse::Found().append_header(("Location", "/homepage")).finish()
}

#[get("/homepage")]
async fn homepage() -> impl Responder {
    HttpResponse::Ok().body(Index {}.render_once().unwrap())
}

#[get("/count-with-pointer")]
async fn count_with_pointer() -> impl Responder {
    let mut num: u128 = 1;
    let pointer = &mut num as *mut u128; 

    let start: DateTime<Utc> = Utc::now();
    with_pointer(pointer);
    let end: DateTime<Utc> = Utc::now();

    unsafe {
        HttpResponse::Ok().body(format!("{}\nTime: {}", *pointer, end - start))
    }
}

/*#[get("/count-with-link")]
async fn count_with_link() -> impl Responder {
    let mut num: u128 = 1;
    let link: &u128 = &num;
    let start: DateTime<Utc> = Utc::now();
    with_link(link);
    let end: DateTime<Utc> = Utc::now();

    HttpResponse::Ok().body(format!("{}\nTime: {}", num, end - start))
}*/

#[get("/count")]
async fn count() -> impl Responder {
    let start: DateTime<Utc> = Utc::now();
    let num: u128 = without_pointer();
    let end: DateTime<Utc> = Utc::now();

    HttpResponse::Ok().body(format!("{}\nTime: {}", num, end - start))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(index_redirect)
            .service(homepage)
            .service(count)
            .service(count_with_pointer)
            //.service(count_with_link)
    }).bind(("127.0.0.1", 8000))?.run().await
}