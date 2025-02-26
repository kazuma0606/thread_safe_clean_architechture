// use crate::application::dto::user::create_user_request_dto::CreateUserRequestDto;
// use crate::application::dto::user::create_user_response_dto::CreateUserResponseDto;
// use crate::application::use_cases::user::create_user_usecase_interface::CreateUserUsecaseInterface;
// use crate::state::app_store::AppStore;
// use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
// use std::sync::Arc;

// pub struct MutationRoot<T: CreateUserUsecaseInterface> {
//     pub user_usecase: Arc<T>,
//     pub store: Arc<AppStore>,
// }

// #[Object]
// impl<T: CreateUserUsecaseInterface + Send + Sync + 'static> MutationRoot<T> {
//     async fn create_user(&self, input: CreateUserRequestDto) -> CreateUserResponseDto {
//         let user = self.user_usecase.execute(input).await;
//         self.store.add_user(user.clone()).await;
//         user
//     }
// }

// pub type AppSchema<T> = Schema<EmptyMutation, MutationRoot<T>, EmptySubscription>;

// pub fn create_schema<T: CreateUserUsecaseInterface + Send + Sync>(
//     usecase: Arc<T>,
//     store: Arc<AppStore>,
// ) -> AppSchema<T> {
//     Schema::build(
//         EmptyMutation,
//         MutationRoot {
//             user_usecase: usecase,
//             store,
//         },
//         EmptySubscription,
//     )
//     .finish()
// }
