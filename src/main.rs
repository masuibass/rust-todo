use actix_web::{HttpResponse, HttpServer, App, get, ResponseError};

use thiserror::Error;

// エラーをまとめるEnumを定義する
// actix_web::ResponseErrorとして使うためにderiveマクロでDebugを付与
#[derive(Error, Debug)]
enum MyError {}

// actix_web::ResponseErrorをMyErrorに実装する
// 今回はデフォルトの実装をそのまま使うので、新たに実装するものはない
impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
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
