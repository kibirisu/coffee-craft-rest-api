use crate::schema::*;
// pub use diesel::sqlite::sql_types::*;
pub use diesel::pg::sql_types::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Selectable, Identifiable)]
pub struct Coffee {
    pub id: i32,
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

#[derive(Insertable)]
#[diesel(table_name = coffees)]
pub struct NewCoffee<'a> {
    pub name: &'a str,
    pub manufacturer: &'a str,
    pub country: &'a str,
    pub processing: &'a str,
    pub package: &'a i32,
    pub price: &'a str,
    pub url: &'a str,
    pub image_url: &'a str,
    pub available: &'a bool,
}

#[derive(Queryable, Selectable, Identifiable, Associations, PartialEq, Deserialize, Serialize)]
#[diesel(belongs_to(Coffee))]
#[diesel(table_name = collection)]
pub struct Collection {
    pub id: i32,
    pub is_stored: bool,
    pub is_favorite: bool,
    pub amount_left: i32,
    pub user_id: String,
    pub coffee_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = collection)]
pub struct NewCollection<'a> {
    pub is_stored: &'a bool,
    pub is_favorite: &'a bool,
    pub amount_left: &'a i32,
    pub user_id: &'a str,
    pub coffee_id: &'a i32,
}
