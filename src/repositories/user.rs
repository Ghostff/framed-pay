use diesel::dsl::exists;
use diesel::prelude::*;
use diesel::{insert_into, result, select};
use uuid::Uuid;
use models::users::User;
use crate::{DBPool, models};
use crate::models::schema::users::dsl::*;

const DB_ERROR: &str = "Couldn't get db connection from pool";

pub fn get_by_id(pool: DBPool, uid: Uuid) -> QueryResult<User> {
    let conn = &mut pool.get().expect(DB_ERROR);
    users.find(uid).first::<User>(conn)
}

pub fn get_by_email(pool: &DBPool, email_address: &str) -> QueryResult<User> {
    let conn = &mut pool.get().expect(DB_ERROR);
    users.filter(email.eq(email_address)).first::<User>(conn)
}

pub fn update(pool: &DBPool, user: &User) -> QueryResult<usize> {
    let conn = &mut pool.get().expect(DB_ERROR);
    diesel::update(users.find(user.id)).set(user).execute(conn)
}

pub fn find_by_password_reset_token(pool: &DBPool, token: String, uid: Uuid) -> QueryResult<User> {
    let conn = &mut pool.get().expect(DB_ERROR);
    users.filter(
        id.eq(uid).and(password_reset_token.like(format!("{token}|%"))),
    ).first::<User>(conn)
}

pub fn email_exist(pool: &DBPool, email_address: &str) -> bool {
    let conn = &mut pool.get().expect(DB_ERROR);
    match select(exists(users.filter(email.eq(email_address)))).get_result(conn) {
        Ok(exists) => exists,
        Err(_) => false,
    }
}

pub fn create(pool: &DBPool, f_name: &str, l_name: &str, email_address: &str, pass: &str) -> bool {
    let conn = &mut pool.get().expect(DB_ERROR);
    insert_into(users)
        .values((
            first_name.eq(f_name),
            last_name.eq(l_name),
            email.eq(email_address),
            password.eq(pass)
        ))
        .execute(conn)
        .is_ok()
}

pub fn create_and_get(pool: &DBPool, f_name: &str, l_name: &str, email_address: &str, pass: &str) -> QueryResult<User> {
    let conn = &mut pool.get().expect(DB_ERROR);
    insert_into(users).values((
        first_name.eq(f_name),
        last_name.eq(l_name),
        email.eq(email_address),
        password.eq(pass)
    )).get_result::<User>(conn)
}
