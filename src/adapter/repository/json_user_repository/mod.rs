use crate::domain::entity::user::User;
use crate::domain::repository::user_repository_interface::UserRepositoryInterface;
use async_trait::async_trait;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct JsonUserRepository {
    users: Arc<Mutex<Vec<User>>>,
    file_path: String,
}

impl JsonUserRepository {
    pub fn new() -> Self {
        let file_path = "users.json".to_string();

        // ファイルから既存のユーザーを読み込む
        let users = Self::load_users(&file_path).unwrap_or_else(|_| Vec::new());

        Self {
            users: Arc::new(Mutex::new(users)),
            file_path,
        }
    }

    // ファイルからユーザーを読み込む
    fn load_users(file_path: &str) -> io::Result<Vec<User>> {
        if !Path::new(file_path).exists() {
            return Ok(Vec::new());
        }

        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        if content.is_empty() {
            return Ok(Vec::new());
        }

        let users: Vec<User> = serde_json::from_str(&content)?;
        Ok(users)
    }

    // ユーザーをファイルに保存
    async fn save_users(&self) -> io::Result<()> {
        let users = self.users.lock().await;
        let json = serde_json::to_string_pretty(&*users)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;

        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

#[async_trait]
impl UserRepositoryInterface for JsonUserRepository {
    async fn create(&self, user: User) -> User {
        let mut users = self.users.lock().await;
        users.push(user.clone());
        drop(users); // ロックを解放

        // ファイルに保存
        if let Err(e) = self.save_users().await {
            eprintln!("ユーザー保存エラー: {}", e);
        }

        user
    }

    async fn find_by_id(&self, id: Uuid) -> Option<User> {
        let users = self.users.lock().await;
        users.iter().cloned().find(|user| user.id() == id)
    }

    async fn update(&self, user: User) -> User {
        let mut users = self.users.lock().await;
        if let Some(existing_user) = users.iter_mut().find(|u| u.id() == user.id()) {
            *existing_user = user.clone();
        }
        drop(users); // ロックを解放

        // ファイルに保存
        if let Err(e) = self.save_users().await {
            eprintln!("ユーザー更新エラー: {}", e);
        }

        user
    }
}
