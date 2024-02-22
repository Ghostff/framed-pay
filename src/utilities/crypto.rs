use argon2::{Argon2, password_hash::{PasswordHash, PasswordVerifier, SaltString}, PasswordHasher};

pub fn hash_password(string: &String) -> String {
    let salt = SaltString::generate(&mut rand_core::OsRng);
    Argon2::default()
        .hash_password(string.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string()
}

pub fn verify_password(old_password: &str, new_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(old_password).unwrap();

    Argon2::default().verify_password(new_password.as_bytes(), &parsed_hash).is_ok()
}