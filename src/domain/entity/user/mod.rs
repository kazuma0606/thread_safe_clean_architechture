use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: Uuid,
    name: String,
    email: String,
    password: String,
    address: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: Uuid, name: String, email: String, password: String, address: String) -> Self {
        Self {
            id,
            name,
            email,
            password,
            address,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn update_email(&mut self, new_email: String) {
        self.email = new_email;
        self.updated_at = Utc::now();
    }

    pub fn update_address(&mut self, new_address: String) {
        self.address = new_address;
        self.updated_at = Utc::now();
    }
}
