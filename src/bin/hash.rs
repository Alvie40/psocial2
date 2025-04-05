use argon2::{Argon2, PasswordHasher};
use rand_core::OsRng;
use argon2::password_hash::SaltString;


fn main() {
    let password = "clt345";
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    println!("Hash gerado: {}", password_hash);
}
