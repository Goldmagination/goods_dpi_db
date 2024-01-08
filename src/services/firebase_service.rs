use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const FIREBASE_SIGN_UP_URL: &str = "https://identitytoolkit.googleapis.com/v1/accounts:signUp";
const FIREBASE_VALIDATE_TOKEN_URL: &str = "https://identitytoolkit.googleapis.com/v1/accounts:lookup";
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


pub async fn create_firebase_user(email: &str, password: &str) -> Result<FirebaseRegisterResponse, String> {
    let api_key = env::var("FIREBASE_API_KEY")
        .expect("FIREBASE_API_KEY must be set");
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
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<FirebaseRegisterResponse>().await {
                    Ok(data) => Ok(data),
                    Err(_) => Err("Failed to parse Firebase response".to_string()),
                }
            } else {
                // You might want to log the actual error or response body for debugging
                Err("Failed to create user in Firebase".to_string())
            }
        },
        Err(_) => Err("Failed to send request to Firebase".to_string()),
    }
}
pub async fn verify_token(token: &str) -> Result<bool, reqwest::Error> {
    let api_key = env::var("FIREBASE_API_KEY")
        .expect("FIREBASE_API_KEY must be set");
    let url = format!("{}?key={}", FIREBASE_VALIDATE_TOKEN_URL, api_key);
    let client = reqwest::Client::new();
    let res = client.post(&url)
        .json(&serde_json::json!({
            "idToken": token
        }))
        .send()
        .await?;

    if res.status().is_success() {
        Ok(true) // Token is valid
    } else {
        Ok(false) // Token is invalid
    }
}

