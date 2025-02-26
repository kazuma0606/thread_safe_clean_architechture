use crate::application::dto::user::create_user_request_dto::CreateUserRequestDto;
use crate::application::dto::user::create_user_response_dto::CreateUserResponseDto;

#[async_trait::async_trait]
pub trait CreateUserUsecaseInterface {
    async fn execute(&self, request: CreateUserRequestDto) -> CreateUserResponseDto;
}
