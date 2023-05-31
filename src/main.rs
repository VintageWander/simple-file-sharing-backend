#![allow(dead_code, unused_variables)]

use std::{net::SocketAddr, sync::Arc};

use aws::S3;
use axum::response::Response;
use config::Config;
use error::Error;
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
    pub storage: S3,
    pub db: Arc<PrismaClient>,
    pub user_service: UserService,
    pub folder_service: FolderService,
}

type WebResult = std::result::Result<Response, Error>;

#[tokio::main]
async fn main() {
    // unsafe { backtrace_on_stack_overflow::enable() }

    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Cannot connect to Postgres");

    let client = Arc::new(client);

    let state = GlobalState {
        storage: S3::init(),
        db: client.clone(),
        user_service: UserService::init(&client),
        folder_service: FolderService::init(&client),
    };

    let Config { port, origin, .. } = Config::from_env();

    let routes = routes().with_state(state).layer(Config::setup_cors(origin));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("Server crashed")
}
