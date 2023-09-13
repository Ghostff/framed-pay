use diesel::dsl::exists;
use diesel::prelude::*;
use diesel::{insert_into, select};
use uuid::Uuid;
use models::users::User;
use crate::{DBPool, models};
use crate::models::schema::users::dsl::*;

pub fn get_by_id(pool: DBPool, uid: Uuid) -> QueryResult<User> {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    users.find(uid).first::<User>(conn)
}

pub fn get_by_email(pool: &DBPool, email_address: &str) -> QueryResult<User> {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    users.filter(email.eq(email_address)).first::<User>(conn)
}

pub fn email_exist(pool: &DBPool, email_address: &str) -> bool {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    match select(exists(users.filter(email.eq(email_address)))).get_result(conn) {
        Ok(exists) => exists,
        Err(_) => false,
    }
}

pub fn create(pool: &DBPool, f_name: &str, l_name: &str, email_address: &str, pass: &str) -> bool {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");
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
    let conn = &mut pool.get().expect("couldn't get db connection from pool");
    insert_into(users).values((
        first_name.eq(f_name),
        last_name.eq(l_name),
        email.eq(email_address),
        password.eq(pass)
    )).get_result::<User>(conn)
}
