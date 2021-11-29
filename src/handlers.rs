use crate::models::Book;
use actix_web::{web, HttpResponse, Responder};
// use serde_json::json;

pub async fn get_all_books() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn create_book(book: web::Json<Book>) -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", book))
}
