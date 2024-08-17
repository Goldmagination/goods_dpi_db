use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, error::Error};
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum FirebaseServiceError {
    #[error("Firebase API error: {0}")]
    FirebaseApiError(String),
    #[error("Failed to decode JWT token")]
    JwtDecodeError,
    #[error("JWT header missing 'kid'")]
    MissingKidError,
    #[error("Invalid 'kid' in JWT header")]
    InvalidKidError,
    #[error("Environment variable not found: {0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Decoding key error: {0}")]
    DecodingKeyError(#[from] jsonwebtoken::errors::Error),
}

pub async fn create_firebase_user(
    email: &str,
    password: &str,
) -> Result<FirebaseRegisterResponse, FirebaseServiceError> {
    let api_key = env::var("FIREBASE_API_KEY")?;
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
        .await?;

    if response.status().is_success() {
        response
            .json::<FirebaseRegisterResponse>()
            .await
            .map_err(|_| {
                FirebaseServiceError::FirebaseApiError(
                    "Failed to parse Firebase response".to_string(),
                )
            })
    } else {
        Err(FirebaseServiceError::FirebaseApiError(
            "Failed to create user in Firebase".to_string(),
        ))
    }
}

pub async fn verify_token(token: &str) -> Result<bool, FirebaseServiceError> {
    let api_key = env::var("FIREBASE_API_KEY")?;
    let url = format!("{}?key={}", FIREBASE_VALIDATE_TOKEN_URL, api_key);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .json(&serde_json::json!({
            "idToken": token
        }))
        .send()
        .await?;

    Ok(res.status().is_success())
}

async fn get_firebase_public_keys() -> Result<HashMap<String, DecodingKey>, FirebaseServiceError> {
    let jwks_url =
        "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";
    let client = Client::new();
    let response = client
        .get(jwks_url)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let mut keys = HashMap::new();
    for (kid, key) in response {
        keys.insert(kid, DecodingKey::from_rsa_pem(key.as_bytes())?);
    }

    Ok(keys)
}

pub async fn extract_uid_from_firebase_token(token: &str) -> Result<String, FirebaseServiceError> {
    let header = decode_header(token).map_err(|_| FirebaseServiceError::JwtDecodeError)?;
    let kid = header.kid.ok_or(FirebaseServiceError::MissingKidError)?;

    let keys = get_firebase_public_keys().await?;
    let key = keys
        .get(&kid)
        .ok_or(FirebaseServiceError::InvalidKidError)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[env::var("FIREBASE_PROJECT_ID")?]);
    validation.set_issuer(&[format!(
        "https://securetoken.google.com/{}",
        env::var("FIREBASE_PROJECT_ID")?
    )]);

    let token_data = decode::<Claims>(token, key, &validation)
        .map_err(|_| FirebaseServiceError::JwtDecodeError)?;

    Ok(token_data.claims.sub)
}

pub async fn upload_image_to_firebase(
    image_bytes: Vec<u8>,
    file_name: String,
) -> Result<String, FirebaseServiceError> {
    let bucket_name = env::var("FIREBASE_BUCKET_NAME")?;
    let url = format!(
        "https://firebasestorage.googleapis.com/v0/b/{}/o?name={}",
        bucket_name, file_name
    );

    let client = Client::new();
    let response = client
        .post(&url)
        .body(image_bytes)
        .header("Content-Type", "image/png")
        .send()
        .await?;

    if response.status().is_success() {
        // Parsing the response to get the image download URL
        let json_response: serde_json::Value = response.json().await?;
        if let Some(download_url) = json_response["downloadTokens"].as_str() {
            Ok(format!(
                "https://firebasestorage.googleapis.com/v0/b/{}/o/{}?alt=media&token={}",
                bucket_name, file_name, download_url
            ))
        } else {
            Err(FirebaseServiceError::FirebaseApiError(
                "Failed to retrieve download URL from Firebase".to_string(),
            ))
        }
    } else {
        Err(FirebaseServiceError::FirebaseApiError(
            "Failed to upload image to Firebase".to_string(),
        ))
    }
}
