use actix_web::{App, Error, HttpResponse, HttpServer, get};
use std::fs;

#[get("/")]
async fn home() -> Result<HttpResponse, Error> {
    let html = fs::read_to_string("static/index.html").map_err(Error::from)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
#[get("/reg")]
async fn register() -> Result<HttpResponse, Error> {
    let html = fs::read_to_string("static/register.html").map_err(Error::from)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
#[get("/login")]
async fn login() -> Result<HttpResponse, Error> {
    let html = fs::read_to_string("static/login.html").map_err(Error::from)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home).service(register).service(login))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
