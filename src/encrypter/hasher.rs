use argon2::{
    self,
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2,
};

pub fn hash(pass: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash: String = argon2.hash_password(pass.as_bytes(), &salt)?.to_string();

    Ok(password_hash)
}
