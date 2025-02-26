use crate::domain::entity::user::User;
use crate::domain::repository::user_repository_interface::UserRepositoryInterface;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

pub struct UserController<T: UserRepositoryInterface> {
    user_repository: T,
}

impl<T: UserRepositoryInterface> UserController<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }
    pub async fn add(&self, user: web::Json<User>) -> impl Responder {
        let new_user = self.user_repository.create(user.into_inner()).await;
        HttpResponse::Ok().json(new_user)
    }

    pub async fn find_by_id(&self, id: web::Path<Uuid>) -> impl Responder {
        let user = self.user_repository.find_by_id(id.into_inner()).await;
        match user {
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::NotFound().json("ユーザーが見つかりませんでした"),
        }
    }
}
