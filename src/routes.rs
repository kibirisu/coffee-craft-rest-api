use actix_web::{error, web, Error, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::{db, Pool};

#[derive(Serialize, Deserialize)]
pub struct CoffeeRequest {
    pub name: String,
    pub manufacturer: String,
    pub country: String,
    pub processing: String,
    pub package: i32,
    pub price: String,
    pub url: String,
    pub image_url: String,
    pub available: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionRequest {
    pub is_stored: bool,
    pub is_favorite: bool,
    pub amount_left: i32,
    pub user_id: i32,
    pub coffee_id: i32,
}

pub async fn get_all_coffees(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let coffee = web::block(move || db::get_coffees(&mut pool.get().unwrap()))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(coffee))
}

pub async fn get_coffee_by_id(
    coffee_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let coffee =
        web::block(move || db::get_coffee(coffee_id.into_inner(), &mut pool.get().unwrap()))
            .await?
            .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(coffee))
}

pub async fn post_coffee(
    new_coffee: web::Json<CoffeeRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || db::insert_coffee(new_coffee, &mut pool.get().unwrap()))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_coffee(
    coffee_id: web::Path<i32>,
    new_coffee: web::Json<CoffeeRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        db::update_coffee(coffee_id.into_inner(), new_coffee, &mut pool.get().unwrap())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn make_coffee_unavailable(
    coffee_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        db::make_coffee_unavailable(coffee_id.into_inner(), &mut pool.get().unwrap())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_coffee(
    coffee_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let result =
        web::block(move || db::delete_coffee(coffee_id.into_inner(), &mut pool.get().unwrap()))
            .await?
            .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_collection(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || db::get_collection(&mut pool.get().unwrap()))
        .await?
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_user_explorer(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_collection =
        web::block(move || db::get_user_explorer(user_id, &mut pool.get().unwrap()))
            .await?
            .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(user_collection))
}

pub async fn get_user_storage(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_storage = web::block(move || db::get_user_storage(user_id, &mut pool.get().unwrap()))
        .await?
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(user_storage))
}

pub async fn get_user_favorites(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_favorites =
        web::block(move || db::get_user_favorites(user_id, &mut pool.get().unwrap()))
            .await?
            .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(user_favorites))
}

pub async fn post_collection(
    new_collection: web::Json<CollectionRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let result =
        web::block(move || db::insert_into_collection(new_collection, &mut pool.get().unwrap()))
            .await?
            .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_collection(
    collection_id: web::Path<i32>,
    new_collection: web::Json<CollectionRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        db::update_collection(
            collection_id.into_inner(),
            new_collection,
            &mut pool.get().unwrap(),
        )
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_collection(
    collection_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        db::delete_collection(collection_id.into_inner(), &mut pool.get().unwrap())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(result))
}
