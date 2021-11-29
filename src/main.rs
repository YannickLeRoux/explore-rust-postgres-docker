#[macro_use]
extern crate log;

use actix_web::{middleware, web, App, HttpServer};
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;
// mod errors;

#[actix_web::main]
async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:test@localhost:5432/my_books")
        .await?;

    let server = HttpServer::new(move || {
        App::new()
            // pass database pool to application so we can access it inside handlers
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
            .route("/book", web::post().to(handlers::create_book))
            .route("/books", web::get().to(handlers::get_all_books))
    })
    .bind("127.0.0.1:8080")?;

    info!("Starting server");
    server.run().await?;
    Ok(())
}
