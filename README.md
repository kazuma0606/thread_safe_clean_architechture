# Rust クリーンアーキテクチャ CLI アプリケーション

このプロジェクトは、Rustでスレッドセーフなクリーンアーキテクチャを実装したCLIアプリケーションです。ユーザー情報をJSONファイルに保存・管理する機能を提供します。

## プロジェクトの概要

このアプリケーションは以下の特徴を持っています：

- クリーンアーキテクチャに基づいた層分け設計
- スレッドセーフな実装（`Arc<Mutex<>>`を活用）
- 依存性注入（DI）パターンの採用
- トレイトを活用したインターフェース分離
- JSONファイルを使用した永続化

## アーキテクチャの概要

プロジェクトは以下の層に分かれています：

1. **ドメイン層** - ビジネスルールとエンティティの定義
2. **アプリケーション層** - ユースケースとDTOの実装
3. **アダプター層** - リポジトリの実装とインフラストラクチャの詳細
4. **フレームワーク層** - CLIインターフェースとメイン関数

### 処理の流れ

1. ユーザーがCLIからコマンドを入力
2. 入力データがDTOに変換される
3. ユースケースが入力DTOを受け取り、ドメインオブジェクトを作成
4. リポジトリを通じてデータが永続化される
5. 結果が出力DTOとして返される
6. 結果がCLIに表示される

## 技術的なポイント

### 1. トレイトを用いたインターフェース分離

```rust
#[async_trait]
pub trait UserRepositoryInterface {
    async fn create(&self, user: User) -> User;
    async fn find_by_id(&self, id: Uuid) -> Option<User>;
    async fn update(&self, user: User) -> User;
}

#[async_trait]
pub trait IdGeneratorInterface {
    fn generate(&self) -> Uuid;
}
```

トレイトを使用することで、具体的な実装から抽象化され、モックオブジェクトによるテストが容易になります。

### 2. 依存性注入（DI）パターン

```rust
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
```

ジェネリクスとトレイト境界を用いて、依存関係を外部から注入できるようにしています。これにより、テストやモックの差し替えが容易になります。

### 3. スレッドセーフな実装

```rust
#[derive(Debug, Clone)]
pub struct JsonUserRepository {
    users: Arc<Mutex<Vec<User>>>,
    file_path: String,
}
```

`Arc<Mutex<>>`を使用することで、複数のスレッドから安全にデータにアクセスできます。`Arc`（Atomic Reference Counting）は参照カウントを原子的に管理し、`Mutex`は排他制御を提供します。

### 4. DTOパターン

```rust
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUserRequestDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUserResponseDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub address: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

データ転送オブジェクト（DTO）を使用して、レイヤー間のデータ受け渡しを行います。これにより、ドメインモデルの変更がインターフェースに影響しにくくなります。

### 5. エンティティの不変性と振る舞い

```rust
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
    
    // ゲッターとセッターメソッド
}
```

エンティティのフィールドを非公開にし、メソッドを通じてのみアクセス可能にすることで、不変性を保ちつつ、ドメインロジックをカプセル化しています。

### 6. 非同期処理

```rust
#[async_trait::async_trait]
impl<T: UserRepositoryInterface + Send + Sync, U: IdGeneratorInterface + Send + Sync>
    CreateUserUsecaseInterface for CreateUserUseCase<T, U>
{
    async fn execute(&self, request_dto: CreateUserRequestDto) -> CreateUserResponseDto {
        // 実装
    }
}
```

`async/await`とトレイトを組み合わせて非同期処理を実装しています。`#[async_trait]`マクロを使用することで、トレイト内での非同期メソッドの定義が可能になります。

### 7. JSONによる永続化

```rust
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
```

`serde_json`を使用して、Rustの構造体とJSONとの間のシリアライズ・デシリアライズを行っています。

## プロジェクト構成

```
src/
├── adapter/
│   ├── repository/
│   │   └── json_user_repository/
│   └── utils/
│       └── uuid_generator/
├── application/
│   ├── dto/
│   │   └── user/
│   │       ├── create_user_request_dto/
│   │       └── create_user_response_dto/
│   └── use_cases/
│       └── user/
│           ├── create_user_usecase/
│           └── create_user_usecase_interface/
├── domain/
│   ├── entity/
│   │   └── user/
│   ├── repository/
│   │   └── user_repository_interface/
│   └── utils/
│       └── id_generator_interface/
├── state/
│   └── app_store/
├── lib.rs
└── main.rs

mod.rsは冗長になるので省略
```

## 開発の流れ

1. **設計フェーズ**:
   - クリーンアーキテクチャの原則に基づいて層構造を設計
   - 各層の責務を明確に分離
   - トレイトとインターフェースを定義

2. **実装フェーズ**:
   - ドメインモデルの実装
   - リポジトリインターフェースの定義
   - ユースケースの実装
   - アダプターの実装（JSONリポジトリ）
   - CLIインターフェースの実装

3. **統合フェーズ**:
   - 依存性注入の設定
   - 非同期処理の統合
   - エラーハンドリングの実装

4. **テストと検証**:
   - 各コンポーネントの動作確認
   - エッジケースのテスト
   - パフォーマンスの検証

## 今後の拡張可能性

- GraphQLインターフェースの追加
- データベースリポジトリの実装
- 認証機能の追加
- WebインターフェースまたはRESTful APIの実装

## まとめ

このプロジェクトは、Rustでクリーンアーキテクチャを実装する方法の一例を示しています。トレイト、ジェネリクス、非同期処理などのRustの機能を活用しながら、保守性が高く、テスト可能なコードを作成することができます。
