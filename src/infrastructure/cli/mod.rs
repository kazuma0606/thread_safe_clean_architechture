use crate::adapter::repository::json_user_repository::JsonUserRepository;
use crate::adapter::utils::uuid_generotor::UuidGenerator;
use crate::application::dto::user::create_user_request_dto::CreateUserRequestDto;
use crate::application::use_cases::user::create_user_usecase::CreateUserUseCase;
use crate::application::use_cases::user::create_user_usecase_interface::CreateUserUsecaseInterface;
use crate::state::app_store::AppStore;
use std::io::{self, Write};
use std::sync::Arc;
pub async fn run() {
    let store = Arc::new(AppStore::new());

    // JsonUserRepositoryを使用
    let user_repository = JsonUserRepository::new();
    let id_generator = UuidGenerator::new();

    // Arc<dyn>パターンで実装
    let create_user_usecase = Arc::new(CreateUserUseCase::new(user_repository, id_generator));

    // 以下のコードは変更なし
    loop {
        println!("コマンドを選択してください:");
        println!("1: ユーザー登録");
        println!("2: ユーザー一覧表示");
        println!("3: 終了");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "1" => {
                // ユーザー登録処理（変更なし）
                println!("ユーザー名を入力してください:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                println!("メールアドレスを入力してください:");
                let mut email = String::new();
                io::stdin().read_line(&mut email).unwrap();
                let email = email.trim().to_string();

                println!("パスワードを入力してください:");
                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();
                let password = password.trim().to_string();

                println!("住所を入力してください:");
                let mut address = String::new();
                io::stdin().read_line(&mut address).unwrap();
                let address = address.trim().to_string();

                let new_user = CreateUserRequestDto {
                    name,
                    email,
                    password,
                    address,
                };

                let created_user = create_user_usecase.execute(new_user).await;
                store.add_user(created_user.clone()).await;
                println!("ユーザーが登録されました: {:?}", created_user);
            }
            "2" => {
                // ユーザー一覧表示機能を追加
                let users = store.get_users().await;
                if users.is_empty() {
                    println!("登録されているユーザーはいません。");
                } else {
                    println!("登録ユーザー一覧:");
                    for (i, user) in users.iter().enumerate() {
                        println!("{}: {} ({})", i + 1, user.name, user.email);
                    }
                }
            }
            "3" => {
                println!("終了します。");
                break;
            }
            _ => println!("無効なコマンドです。"),
        }
    }
}
