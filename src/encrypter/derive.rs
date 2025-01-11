use argon2::{self, password_hash::Error, Argon2};

pub fn derive(pass: &str) -> Result<[u8; 32], Error> {
    let salt = b"MarquinSecureSalt!";

    let mut output_key_material = [0u8; 32];

    Argon2::default().hash_password_into(pass.as_bytes(), salt, &mut output_key_material)?;

    Ok(output_key_material)
}
