use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUserRequestDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub address: String,
}
