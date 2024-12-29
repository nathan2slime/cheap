use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct JwtPayload {
    sub: String,
    exp: usize,
    iat: usize,
}

pub struct Mutation;

impl Mutation {
    pub async fn create(secret: String, sub: String, time: i64) -> Option<String>{
        let now = Utc::now();
        let expiration = now + Duration::days(time);

        let payload = JwtPayload {
            sub,
            iat: now.timestamp() as usize,
            exp: expiration.timestamp() as usize,
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &payload,
            &EncodingKey::from_secret(secret.as_ref()),
        ).unwrap();

        Some(token)
    }
}
