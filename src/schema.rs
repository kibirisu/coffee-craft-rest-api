// @generated automatically by Diesel CLI.

diesel::table! {
    coffees (id) {
        id -> Integer,
        name -> Text,
        manufacturer -> Text,
        country -> Text,
        processing -> Text,
        package -> Integer,
        price -> Text,
        url -> Text,
        image_url -> Text,
        available -> Bool,
    }
}

diesel::table! {
    collection (id) {
        id -> Integer,
        is_stored -> Bool,
        is_favorite -> Bool,
        amount_left -> Integer,
        user_id -> Integer,
        coffee_id -> Integer,
    }
}

diesel::joinable!(collection -> coffees (coffee_id));

diesel::allow_tables_to_appear_in_same_query!(
    coffees,
    collection,
);
