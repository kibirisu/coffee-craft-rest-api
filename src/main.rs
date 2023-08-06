mod db;
mod models;
mod routes;
mod schema;

#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

type DatabaseConnection = PgConnection;
type Pool = r2d2::Pool<ConnectionManager<DatabaseConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // let database_url = std::env::var("DATABASE_URL").unwrap_or("coffee.db".to_string());
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let database_pool = Pool::builder()
        .build(ConnectionManager::<DatabaseConnection>::new(database_url))
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database_pool.clone()))
            .route("/coffees", web::get().to(routes::get_all_coffees))
            .route(
                "/coffees/{cofffe_id}",
                web::get().to(routes::get_coffee_by_id),
            )
            .route("/coffees", web::post().to(routes::post_coffee))
            .route("/coffees", web::put().to(routes::update_coffee))
            .route(
                "/coffees/disable/{coffee_id}",
                web::put().to(routes::make_coffee_unavailable),
            )
            .route(
                "/coffees/{cofffe_id}",
                web::delete().to(routes::delete_coffee),
            )
            .route(
                "/explorer/{user_id}",
                web::get().to(routes::get_user_explorer),
            )
            .route("/collection", web::get().to(routes::get_collection))
            .route(
                "/collection/storage/{user_id}",
                web::get().to(routes::get_user_storage),
            )
            .route(
                "/collection/favorites/{user_id}",
                web::get().to(routes::get_user_favorites),
            )
            .route("/collection", web::post().to(routes::post_collection))
            .route(
                "/collection/{collection_id}",
                web::put().to(routes::update_collection),
            )
            .route(
                "/collection/{collection_id}",
                web::delete().to(routes::delete_collection),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
