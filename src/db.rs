use actix_web::web;
use diesel::prelude::*;

use crate::{
    models::{Coffee, Collection, NewCoffee, NewCollection},
    routes::{CoffeeRequest, CollectionRequest},
    schema::coffees::{self, dsl::*},
    DatabaseConnection,
};

pub fn get_coffees(conn: &mut DatabaseConnection) -> QueryResult<Vec<Coffee>> {
    coffees.load::<Coffee>(conn)
}

pub fn get_coffee(coffee_id: i32, conn: &mut DatabaseConnection) -> QueryResult<Coffee> {
    coffees.find(coffee_id).get_result::<Coffee>(conn)
}

pub fn insert_coffee(
    new_coffee: web::Json<CoffeeRequest>,
    conn: &mut DatabaseConnection,
) -> QueryResult<usize> {
    let coffee = NewCoffee {
        name: &new_coffee.name,
        manufacturer: &new_coffee.manufacturer,
        country: &new_coffee.country,
        processing: &new_coffee.processing,
        package: &new_coffee.package,
        price: &new_coffee.price,
        url: &new_coffee.url,
        image_url: &new_coffee.image_url,
        available: &new_coffee.available,
    };
    diesel::insert_into(coffees).values(&coffee).execute(conn)
}

pub fn update_coffee(
    coffee_id: i32,
    new_coffee: web::Json<CoffeeRequest>,
    conn: &mut DatabaseConnection,
) -> QueryResult<usize> {
    diesel::update(coffees)
        .filter(coffees::id.eq(coffee_id))
        .set(coffees::name.eq(&new_coffee.name))
        .execute(conn)
}

pub fn make_coffee_unavailable(
    coffee_id: i32,
    conn: &mut DatabaseConnection,
) -> QueryResult<usize> {
    diesel::update(coffees)
        .filter(coffees::id.eq(coffee_id))
        .set(coffees::available.eq(false))
        .execute(conn)
}

pub fn delete_coffee(coffee_id: i32, conn: &mut DatabaseConnection) -> QueryResult<usize> {
    diesel::delete(coffees.find(coffee_id)).execute(conn)
}

pub fn get_collection(conn: &mut DatabaseConnection) -> QueryResult<Vec<Collection>> {
    use crate::schema::collection::dsl::*;
    collection.load::<Collection>(conn)
}

pub fn get_user_explorer(
    user: web::Path<String>,
    conn: &mut DatabaseConnection,
) -> QueryResult<Vec<(Coffee, Option<Collection>)>> {
    use crate::schema::collection;
    coffees::table
        .left_join(
            collection::table.on(collection::user_id
                .eq(user.into_inner())
                .and(coffees::id.eq(collection::coffee_id))),
        )
        .select((Coffee::as_select(), Option::<Collection>::as_select()))
        .load::<(Coffee, Option<Collection>)>(conn)
}

pub fn get_user_storage(
    user: web::Path<String>,
    conn: &mut DatabaseConnection,
) -> QueryResult<Vec<(Coffee, Collection)>> {
    use crate::schema::collection;
    collection::table
        .inner_join(coffees::table)
        .filter(collection::user_id.eq(user.into_inner()))
        .filter(collection::is_stored.eq(true))
        .select((Coffee::as_select(), Collection::as_select()))
        .load(conn)
}

pub fn get_user_favorites(
    user: web::Path<String>,
    conn: &mut DatabaseConnection,
) -> QueryResult<Vec<(Coffee, Collection)>> {
    use crate::schema::collection;
    collection::table
        .inner_join(coffees::table)
        .filter(collection::user_id.eq(user.into_inner()))
        .filter(collection::is_favorite.eq(true))
        .select((Coffee::as_select(), Collection::as_select()))
        .load(conn)
}

pub fn insert_into_collection(
    new_collection: web::Json<CollectionRequest>,
    conn: &mut DatabaseConnection,
) -> QueryResult<usize> {
    use crate::schema::collection::dsl::*;
    let collected = NewCollection {
        is_stored: &new_collection.is_stored,
        is_favorite: &new_collection.is_favorite,
        amount_left: &new_collection.amount_left,
        user_id: &new_collection.user_id,
        coffee_id: &new_collection.coffee_id,
    };
    diesel::insert_into(collection)
        .values(&collected)
        .execute(conn)
}

pub fn update_collection(
    collection_id: i32,
    new_collection: web::Json<CollectionRequest>,
    conn: &mut DatabaseConnection,
) -> QueryResult<usize> {
    use crate::schema::collection::{self, dsl::*};
    diesel::update(collection)
        .filter(collection::id.eq(collection_id))
        .set((
            collection::is_stored.eq(new_collection.is_stored),
            collection::is_favorite.eq(new_collection.is_favorite),
        ))
        .execute(conn)
}

pub fn delete_collection(collection_id: i32, conn: &mut DatabaseConnection) -> QueryResult<usize> {
    use crate::schema::collection::dsl::*;
    diesel::delete(collection.find(collection_id)).execute(conn)
}
