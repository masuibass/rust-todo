use actix_web::{HttpResponse, HttpServer, App, get};

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello world!";

    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
