mod domain;
mod repository;
mod config;

use std::process::exit;
use dotenv::dotenv;
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};

use greeter::greeter_server::{Greeter, GreeterServer};
use greeter::{HelloResponse, HelloRequest};
use uuid::Uuid;
use crate::domain::User;
use crate::repository::{PostgresUserRepository, UserRepository};

// Импортируем сгенерированный proto файл в модуль
pub mod greeter {
    tonic::include_proto!("greeter");
}

pub struct MyGreeter {
    user_repo: Arc<dyn UserRepository>,
}

// Реализуем конструктор для MyGreeter
impl MyGreeter {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}

// Реализуем функции сервиса, определенные в proto
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Received request from: {:?}", request);

        let users = self.user_repo.users().await;


        // Создаем новый объект User
        let new_user = User {
            id: Uuid::new_v4(),
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
        };

        // Пример создания нового пользователя
        match self.user_repo.create(new_user).await {
            Ok(user) => println!("Created user: {:?}", user),
            Err(e) => eprintln!("Error creating user: {:?}", e),
        }

        let user_list = match users {
            Ok(users) => users,
            Err(_) => exit(0),
        };

        for user in user_list {
            println!("{}", user.id)
        }

        let response = greeter::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(response))
    }
}

// Запускаем сервер
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Создаем пул соединений с базой данных
    let pool = Arc::new(config::create_pool().await.expect("Failed to create pool"));

    // Создаем репозиторий пользователей
    let user_repo = Arc::new(PostgresUserRepository::new(pool));

    // Создаем gRPC сервер с нашим Greeter
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::new(user_repo.clone());

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
