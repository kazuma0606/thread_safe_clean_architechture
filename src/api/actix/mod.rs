// use crate::{
//     api::schema::{AppSchema, create_schema},
//     state::app_store::AppStore,
// };
// use actix_web::{App, HttpResponse, HttpServer, Responder, web};
// use async_graphql::{Request, Response, Schema};
// use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
// use std::sync::Arc;

// pub async fn graphql_handler<T: CreateUserUseCaseInterfac + Send + Sync>(
//     schema: web::Data<AppSchema<T>>,
//     req: GraphQLRequest,
// ) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let store = Arc::new(AppStore::new());
//     let usecase =
//         Arc::new(crate::application::use_cases::user::create_user_usecase::new(store.clone()));
//     let schema = create_schema(usecase.clone(), store.clone());

//     HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::new(schema.clone()))
//             .route("/graphql", web::post().to(graphql_handler::<_>))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
