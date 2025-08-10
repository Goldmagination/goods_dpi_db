use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user_id
    pub email: String,
    pub exp: usize,   // expiration time
    pub iat: usize,   // issued at
}

impl Claims {
    pub fn new(user_id: String, email: String) -> Self {
        let now = Utc::now();
        let exp = (now + chrono::Duration::days(7)).timestamp() as usize;
        let iat = now.timestamp() as usize;
        
        Claims {
            sub: user_id,
            email,
            exp,
            iat,
        }
    }
}