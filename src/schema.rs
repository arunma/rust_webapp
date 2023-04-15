// @generated automatically by Diesel CLI.

diesel::table! {
    cookies (id) {
        id -> Int4,
        name -> Varchar,
        image_path -> Varchar,
    }
}
