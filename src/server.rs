mod app;
mod config;
mod di;
mod domain;
mod repository;

use dotenv::dotenv;
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};

use crate::app::command::create_short_url::CreateShortUrlRepository;
use crate::app::query::get_full_url::GetFullUrlRepository;
use crate::di::Container;
use crate::greeter::{
    CreateShortUrlRequest, CreateShortUrlResponse, GetUrlRequest, GetUrlResponse,
};
use crate::repository::postgres::url_repository::url_repository::PostgresUrlRepository;
use greeter::greeter_server::{Greeter, GreeterServer};

pub mod greeter {
    tonic::include_proto!("greeter");
}

pub struct MyGreeter<R, I>
where
    R: CreateShortUrlRepository + Send + Sync + 'static,
    I: GetFullUrlRepository + Send + Sync + 'static,
{
    container: Arc<Container<R, I>>,
}

impl<R, I> MyGreeter<R, I>
where
    R: CreateShortUrlRepository + Send + Sync + 'static,
    I: GetFullUrlRepository + Send + Sync + 'static,
{
    pub fn new(container: Arc<Container<R, I>>) -> Self {
        MyGreeter { container }
    }
}

#[tonic::async_trait]
impl<R, I> Greeter for MyGreeter<R, I>
where
    R: CreateShortUrlRepository + Send + Sync + 'static,
    I: GetFullUrlRepository + Send + Sync + 'static,
{
    async fn create_short_url(
        &self,
        request: Request<CreateShortUrlRequest>,
    ) -> Result<Response<CreateShortUrlResponse>, Status> {
        let req = request.get_ref();
        let url_result = self
            .container
            .shorten_command
            .execute(req.full_url.clone())
            .await;

        match url_result {
            Ok(short_url) => Ok(Response::new(CreateShortUrlResponse { short_url })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_url(
        &self,
        request: Request<GetUrlRequest>,
    ) -> Result<Response<GetUrlResponse>, Status> {
        let req = request.get_ref();
        let url_result = self
            .container
            .full_url_query
            .execute(req.clone().short_url)
            .await;

        match url_result {
            Ok(url) => Ok(Response::new(GetUrlResponse {
                full_url: url.url_full,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = config::create_pool().await.expect("Failed to create pool");
    let pool = Arc::new(pool);

    // let user_repo = Arc::new(PostgresUserRepository::new(Arc::clone(&pool)));
    let repo = PostgresUrlRepository::new(Arc::clone(&pool));
    let container = Container::new(repo.clone(), repo);
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::new(Arc::new(container));

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
