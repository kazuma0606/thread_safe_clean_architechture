use crate::application::dto::user::create_user_request_dto::CreateUserRequestDto;
use crate::application::dto::user::create_user_response_dto::CreateUserResponseDto;
use crate::application::use_cases::user::create_user_usecase_interface::CreateUserUsecaseInterface;
use crate::domain::entity::user::User;
use crate::domain::repository::user_repository_interface::UserRepositoryInterface;
use crate::domain::utils::id_generator_interaface::IdGeneratorInterface;

pub struct CreateUserUseCase<T: UserRepositoryInterface, U: IdGeneratorInterface> {
    user_repository: T,
    id_generator: U,
}

impl<T: UserRepositoryInterface, U: IdGeneratorInterface> CreateUserUseCase<T, U> {
    pub fn new(user_repository: T, id_generator: U) -> Self {
        Self {
            user_repository,
            id_generator,
        }
    }
}

#[async_trait::async_trait]
impl<T: UserRepositoryInterface + Send + Sync, U: IdGeneratorInterface + Send + Sync>
    CreateUserUsecaseInterface for CreateUserUseCase<T, U>
{
    async fn execute(&self, request_dto: CreateUserRequestDto) -> CreateUserResponseDto {
        let id = self.id_generator.generate();

        // User エンティティを作成
        let user = User::new(
            id,
            request_dto.name.clone(),
            request_dto.email.clone(),
            request_dto.password.clone(),
            request_dto.address.clone(),
        );

        // リポジトリにユーザーを保存
        self.user_repository.create(user.clone()).await;

        // レスポンスDTOを返す
        CreateUserResponseDto {
            id: user.id(),
            name: request_dto.name,
            email: request_dto.email,
            password: request_dto.password,
            address: request_dto.address,
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}
