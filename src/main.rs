#![allow(dead_code, unused_variables)]

use std::{net::SocketAddr, sync::Arc};

use aws::S3;
use axum::response::Response;

use config::{check_env, setup_cors, PORT};
use error::Error;
use file::service::FileService;
use folder::service::FolderService;
use prisma::PrismaClient;

use routes::routes;
use user::service::UserService;

mod auth;
mod aws;
mod error;
mod extractors;
mod folder;

mod config;
mod file;
mod impls;
#[allow(warnings)]
mod prisma;
mod routes;
mod user;
mod validation;
mod web;

#[derive(Clone)]
pub struct GlobalState {
    pub db: Arc<PrismaClient>,
    pub user_service: UserService,
    pub folder_service: FolderService,
    pub file_service: FileService,
    pub storage: S3,
}

type WebResult = std::result::Result<Response, Error>;

#[tokio::main]
async fn main() {
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
        storage: S3::init(),
    };

    let routes = routes().with_state(state).layer(setup_cors());

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("Server crashed")
}
