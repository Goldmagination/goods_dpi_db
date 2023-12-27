use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegistrationData {
    pub email: String,
    pub password: String,
    pub name: String,
}
