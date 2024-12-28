use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub struct Mutation;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct JwtPayload {
    sub: String,
    iat: i64,
    exp: i64,
}

impl Mutation {
    pub async fn create(sub: &str, exp: &i64, session_secret: String) {
        let clains = JwtPayload {
            sub: sub.to_owned(),
            iat: Utc::now().timestamp(),
            exp: exp.to_owned(),
        };

        encode(
            &Header::default(),
            &clains,
            &EncodingKey::from_secret(session_secret.as_ref()),
        )
        .unwrap();
    }
}
