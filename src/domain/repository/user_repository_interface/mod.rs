use crate::domain::entity::user::User;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepositoryInterface {
    async fn create(&self, user: User) -> User;
    async fn find_by_id(&self, id: Uuid) -> Option<User>;
    async fn update(&self, user: User) -> User;
    // async fn delete(&self, id: Uuid) -> Result<(), String>;
}
