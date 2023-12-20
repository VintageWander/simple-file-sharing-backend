use std::sync::Arc;

use aws::S3;
use axum::response::Response;

use config::{check_env, port, setup_cors};
use dotenvy::dotenv;
use error::Error;
use file::service::FileService;
use file_version::service::FileVersionService;
use folder::service::FolderService;
use prisma::PrismaClient;

use routes::routes;
use tag::service::TagService;
use tokio::net::TcpListener;
use user::service::UserService;

mod auth;
mod error;
mod extractors;
mod folder;

mod aws;
mod config;
mod file;
mod file_version;
mod impls;
#[allow(warnings)]
mod prisma;
mod routes;
mod tag;
mod user;
mod validation;
mod web;

#[derive(Clone)]
pub struct GlobalState {
    pub db: Arc<PrismaClient>,
    pub user_service: UserService,
    pub folder_service: FolderService,
    pub file_service: FileService,
    pub file_version_service: FileVersionService,
    pub tag_service: TagService,
    pub storage: S3,
}

type WebResult = std::result::Result<Response, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    check_env();

    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Cannot connect to Postgres");

    let client = Arc::new(client);

    let state = GlobalState {
        db: client.clone(),
        user_service: UserService::init(&client),
        folder_service: FolderService::init(&client),
        file_service: FileService::init(&client),
        file_version_service: FileVersionService::init(&client),
        tag_service: TagService::init(&client),
        storage: S3::init(),
    };

    let routes = routes().with_state(state).layer(setup_cors());

    let port = port();

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    axum::serve(listener, routes)
        .await
        .map(|_| {
            println!("Server started at port {port}");
        })
        .expect("Server crashed");
}
