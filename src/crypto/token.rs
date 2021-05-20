use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    name: String,
}

pub fn generate_token(sub: String, name: String) -> String {
    let config = Config::from_env().expect("Secrets configuration");

    let my_claims = Claims { sub, name };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )?;

    return token;
}
