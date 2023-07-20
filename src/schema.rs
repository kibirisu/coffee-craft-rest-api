// @generated automatically by Diesel CLI.

diesel::table! {
    coffees (id) {
        id -> Int4,
        name -> Text,
        manufacturer -> Text,
        country -> Text,
        processing -> Text,
        package -> Int4,
        price -> Text,
        url -> Text,
        image_url -> Text,
        available -> Bool,
    }
}

diesel::table! {
    collection (id) {
        id -> Int4,
        is_stored -> Bool,
        is_favorite -> Bool,
        amount_left -> Int4,
        user_id -> Text,
        coffee_id -> Int4,
    }
}

diesel::joinable!(collection -> coffees (coffee_id));

diesel::allow_tables_to_appear_in_same_query!(
    coffees,
    collection,
);
