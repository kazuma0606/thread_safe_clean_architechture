use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUserResponseDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub address: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
