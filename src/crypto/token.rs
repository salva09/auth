use color_eyre::Result;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Claims {
    sub: String,
    name: String,
}

pub fn generate_token(sub: String, name: String) -> Result<String> {
    let jwt_secret = Config::get("JWT_SECRET".parse()?);

    let my_claims = Claims { sub, name };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?;

    Ok(token)
}
