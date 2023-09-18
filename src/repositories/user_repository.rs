use diesel::prelude::*;
use diesel::{insert_into};
use uuid::Uuid;
use models::user_model::User;
use crate::{DBPool, models};
use crate::models::schema::users::dsl::*;

const DB_ERROR: &str = "Couldn't get db connection from pool";

pub fn get_by_id(pool: &DBPool, uid: &Uuid) -> QueryResult<User> {
    users.find(uid).first::<User>(&mut pool.get().expect(DB_ERROR))
}

pub fn get_by_email(pool: &DBPool, email_address: &str) -> QueryResult<User> {
    users.filter(email.eq(email_address)).first::<User>(&mut pool.get().expect(DB_ERROR))
}

pub fn get_by_api_key(pool: &DBPool, key: &str) -> QueryResult<User> {
    users.filter(api_key.eq(key)).first::<User>(&mut pool.get().expect(DB_ERROR))
}

pub fn update(pool: &DBPool, user: &User) -> QueryResult<usize> {
    diesel::update(users.find(user.id)).set(user).execute(&mut pool.get().expect(DB_ERROR))
}

pub fn find_by_password_reset_token(pool: &DBPool, token: String, uid: Uuid) -> QueryResult<User> {
    users.filter(
        id.eq(uid).and(password_reset_token.like(format!("{token}|%"))),
    ).first::<User>(&mut pool.get().expect(DB_ERROR))
}

pub fn create_and_get(pool: &DBPool, f_name: &str, l_name: &str, email_address: &str, pass: &str, key: &str) -> QueryResult<User> {
    insert_into(users).values((
        first_name.eq(f_name),
        last_name.eq(l_name),
        email.eq(email_address),
        password.eq(pass),
        api_key.eq(key)
    )).get_result::<User>(&mut pool.get().expect(DB_ERROR))
}
