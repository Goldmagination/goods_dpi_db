use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::{collections::HashMap, env, error::Error};

#[derive(Debug)]
struct FirebaseError(String);

impl fmt::Display for FirebaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FirebaseError: {}", self.0)
    }
}

impl Error for FirebaseError {}

impl Send for FirebaseError {}
impl Sync for FirebaseError {}

const FIREBASE_SIGN_UP_URL: &str = "https://identitytoolkit.googleapis.com/v1/accounts:signUp";
const FIREBASE_VALIDATE_TOKEN_URL: &str =
    "https://identitytoolkit.googleapis.com/v1/accounts:lookup";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

#[derive(Serialize)]
struct FirebaseRegisterRequest {
    email: String,
    password: String,
    return_secure_token: bool,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct FirebaseRegisterResponse {
    pub idToken: String,
    pub email: String,
    pub refreshToken: String,
    pub expiresIn: String,
    pub localId: String, // The UID
}

pub async fn create_firebase_user(
    email: &str,
    password: &str,
) -> Result<FirebaseRegisterResponse, Box<dyn Error + Send + Sync>> {
    let api_key = env::var("FIREBASE_API_KEY").expect("FIREBASE_API_KEY must be set");
    let client = Client::new();
    let request_body = FirebaseRegisterRequest {
        email: email.to_string(),
        password: password.to_string(),
        return_secure_token: true,
    };

    let response = client
        .post(format!("{}?key={}", FIREBASE_SIGN_UP_URL, api_key))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| Box::new(FirebaseError(e.to_string())) as Box<dyn Error + Send + Sync>)?;

    if response.status().is_success() {
        response
            .json::<FirebaseRegisterResponse>()
            .await
            .map_err(|e| Box::new(FirebaseError(e.to_string())) as Box<dyn Error + Send + Sync>)
    } else {
        Err(
            Box::new(FirebaseError("Failed to create user in Firebase".into()))
                as Box<dyn Error + Send + Sync>,
        )
    }
}

pub async fn verify_token(token: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let api_key = env::var("FIREBASE_API_KEY").expect("FIREBASE_API_KEY must be set");
    let url = format!("{}?key={}", FIREBASE_VALIDATE_TOKEN_URL, api_key);
    let client = Client::new();
    let res = client
        .post(&url)
        .json(&serde_json::json!({
            "idToken": token
        }))
        .send()
        .await
        .map_err(|e| Box::new(FirebaseError(e.to_string())) as Box<dyn Error + Send + Sync>)?;

    Ok(res.status().is_success())
}

async fn get_firebase_public_keys(
) -> Result<HashMap<String, DecodingKey>, Box<dyn Error + Send + Sync>> {
    let jwks_url =
        "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";
    let client = Client::new();
    let response = client
        .get(jwks_url)
        .send()
        .await
        .map_err(|e| Box::new(FirebaseError(e.to_string())) as Box<dyn Error + Send + Sync>)?
        .json::<HashMap<String, String>>()
        .await
        .map_err(|e| Box::new(FirebaseError(e.to_string())) as Box<dyn Error + Send + Sync>)?;

    let mut keys = HashMap::new();
    for (kid, key) in response {
        keys.insert(kid, DecodingKey::from_rsa_pem(key.as_bytes())?);
    }

    Ok(keys)
}

pub async fn extract_uid_from_firebase_token(
    token: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let header = decode_header(token)
        .map_err(|e| Box::new(FirebaseError(e.to_string())) as Box<dyn Error + Send + Sync>)?;
    let kid = header.kid.ok_or_else(|| {
        Box::new(FirebaseError("Missing 'kid' in JWT header".to_string()))
            as Box<dyn Error + Send + Sync>
    })?;

    let keys = get_firebase_public_keys().await?;
    let key = keys.get(&kid).ok_or_else(|| {
        Box::new(FirebaseError("Invalid 'kid' in JWT header".to_string()))
            as Box<dyn Error + Send + Sync>
    })?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation
        .set_audience(&[env::var("FIREBASE_PROJECT_ID").expect("FIREBASE_PROJECT_ID must be set")]);
    validation.set_issuer(&[format!(
        "https://securetoken.google.com/{}",
        env::var("FIREBASE_PROJECT_ID").expect("FIREBASE_PROJECT_ID must be set")
    )]);

    let token_data = decode::<Claims>(token, key, &validation)
        .map_err(|e| Box::new(FirebaseError(e.to_string())) as Box<dyn Error + Send + Sync>)?;

    Ok(token_data.claims.sub)
}

pub async fn upload_image_to_firebase(
    image_bytes: Vec<u8>,
    file_name: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let firebase_storage_bucket =
        env::var("FIREBASE_STORAGE_BUCKET").expect("FIREBASE_STORAGE_BUCKET must be set");
    let upload_url = format!(
        "https://firebasestorage.googleapis.com/v0/b/{}/o?uploadType=media&name={}",
        firebase_storage_bucket, file_name
    );

    let client = Client::new();
    let response = client
        .post(&upload_url)
        .header("Content-Type", "image/jpeg") // Adjust content type if necessary
        .bearer_auth("YOUR_FIREBASE_TOKEN")
        .body(image_bytes)
        .send()
        .await
        .map_err(|e| Box::new(FirebaseError(e.to_string())) as Box<dyn Error + Send + Sync>)?;

    if response.status().is_success() {
        let json_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Box::new(FirebaseError(e.to_string())) as Box<dyn Error + Send + Sync>)?;
        if let Some(url) = json_response["mediaLink"].as_str() {
            return Ok(url.to_string());
        }
    }

    Err(Box::new(FirebaseError("Failed to upload image".into())) as Box<dyn Error + Send + Sync>)
}
