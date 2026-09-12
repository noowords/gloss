use uuid::{ Uuid };
use chrono::{ Utc, Duration };
use serde::{ Serialize, Deserialize };
use jsonwebtoken::{ encode, decode, Header, EncodingKey, DecodingKey, Validation };

use domain::aggregates::user::value_objects::{ UserId, UserRole };
use application::contracts::{ TokenService };

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Claims {
    Access {
        sub: Uuid,
        role: String,
        exp: usize
    },
    Refresh {
        sub: Uuid,
        exp: usize
    }
}

pub struct JwtTokenService {
    secret: String
}

impl JwtTokenService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl TokenService for JwtTokenService {
    fn generate_access_token(&self, user_id: UserId, role: UserRole) -> Result<String, anyhow::Error> {
        let expiration = Utc::now() + Duration::minutes(15);
        
        let claims = Claims::Access {
            sub: user_id.into(),
            role: role.into(),
            exp: expiration.timestamp() as usize
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        ).map_err(|e| anyhow::anyhow!("Failed to generate access token: {}", e))?;
        
        Ok(token)
    }

    fn generate_refresh_token(&self, user_id: UserId) -> Result<String, anyhow::Error> {
        let expiration = Utc::now() + Duration::days(7);
        
        let claims = Claims::Refresh {
            sub: user_id.into(),
            exp: expiration.timestamp() as usize
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        ).map_err(|e| anyhow::anyhow!("Failed to generate refresh token: {}", e))?;
        
        Ok(token)
    }

    fn verify_access_token(&self, token: &str) -> Result<(UserId, UserRole), anyhow::Error> {
        let validation = Validation::default();
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        ).map_err(|e| anyhow::anyhow!("Invalid token: {}", e))?;

        match token_data.claims {
            Claims::Access { sub, role, .. } => {
                let user_id = sub.try_into()
                    .map_err(|_| anyhow::anyhow!("Invalid user id in token"))?;
                let user_role = UserRole::try_from(role)
                    .map_err(|_| anyhow::anyhow!("Invalid user role in token"))?;
                
                Ok((user_id, user_role))
            }
            Claims::Refresh { .. } => {
                Err(anyhow::anyhow!("Expected access token, got refresh token"))
            }
        }
    }

    fn verify_refresh_token(&self, token: &str) -> Result<UserId, anyhow::Error> {
        let validation = Validation::default();
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        ).map_err(|e| anyhow::anyhow!("Invalid token: {}", e))?;

        match token_data.claims {
            Claims::Refresh { sub, .. } => {
                let user_id = UserId::try_from(sub)
                    .map_err(|_| anyhow::anyhow!("Invalid user id in token"))?;
                
                Ok(user_id)
            }
            Claims::Access { .. } => {
                Err(anyhow::anyhow!("Expected refresh token, got access token"))
            }
        }
    }
}
