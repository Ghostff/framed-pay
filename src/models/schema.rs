// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        avatar -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        password_reset_token -> Nullable<Varchar>,
        #[max_length = 100]
        role -> Varchar,
        last_logged_in_at -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}
