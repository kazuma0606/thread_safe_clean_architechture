use crate::application::dto::user::create_user_response_dto::CreateUserResponseDto;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppStore {
    pub users: Arc<Mutex<Vec<CreateUserResponseDto>>>,
}

impl AppStore {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_user(&self, user: CreateUserResponseDto) {
        let mut users = self.users.lock().await;
        users.push(user);
    }

    pub async fn get_users(&self) -> Vec<CreateUserResponseDto> {
        let users = self.users.lock().await;
        users.clone()
    }
}
