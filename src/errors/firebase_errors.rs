use base64::DecodeError as Base64DecodeError;
use jsonwebtoken::errors::Error as JwtError;
use reqwest::Error as ReqwestError;
use std::env::VarError;
use thiserror::Error;

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
    EnvVarError(#[from] VarError),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] ReqwestError),
    #[error("Decoding key error: {0}")]
    DecodingKeyError(#[from] JwtError),
    #[error("Base64 decoding error: {0}")]
    Base64DecodeError(#[from] Base64DecodeError),
}
