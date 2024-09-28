use std::usize;

use chrono::{Duration, Utc};
use super::err::Result;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

use super::domain::Client;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}

pub struct TokenProvider {
    token_key: String
}

impl TokenProvider {
    pub fn new(token_key: String) -> Self {
        Self { token_key }
    }

    pub fn generate_token(&self, client_name: String) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: client_name,
            exp: expiration
        };

        let token = encode (
            &Header::new(jsonwebtoken::Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.token_key.as_ref())
        )?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenData<Claims>> {

        let mut validation =  Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.leeway = 60;

        let token_data = decode(
            token,
            &DecodingKey::from_secret(self.token_key.as_ref()),
            &validation
        )?;

        Ok(token_data)
    }
}

#[test]
fn test_token_provider_validity() {
    let token_provider = TokenProvider::new("gxQy0CBeYonc3UByo72Q24B7K8EizgRo0NfzxMdwEoQ=".to_string());

    let token_result = token_provider.generate_token("esteban".to_string()).unwrap();
    let claims = token_provider.verify_token(&token_result).unwrap();

    assert_eq!(claims.claims.sub, "esteban");
}
