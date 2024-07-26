use std::env;
extern crate argonautica;
use argonautica::Hasher;
use argonautica::Verifier;

fn hash_password(password: &str) -> Result<String, argonautica::Error> {
    let secret_key = env::var("HAS_SECRET").expect("SECRET_KEY must be set");

    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(password)
        .with_secret_key(secret_key)
        .hash()?;
    Ok(hash)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, argonautica::Error> {
    let secret_key = env::var("HAS_SECRET").expect("SECRET_KEY must be set");

    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(secret_key)
        .verify()?;
    Ok(is_valid)
}
