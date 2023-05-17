#![allow(dead_code, unused_variables)]

use std::{net::SocketAddr, sync::Arc};

use auth::controller::AuthController;
use axum::{
    http::{
        header::{
            ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, ORIGIN,
        },
        HeaderValue, Method,
    },
    response::Response,
    Router,
};
use dotenvy::var;
use error::Error;
use file::controller::FileController;
use folder::controller::FolderController;
use prisma::PrismaClient;

use tower_http::cors::CorsLayer;
use user::controller::UserController;

mod auth;
mod error;
mod extractors;
mod folder;

mod file;
#[allow(warnings)]
mod prisma;
mod user;
mod validation;
mod web;

type Database = Arc<PrismaClient>;
type WebResult = std::result::Result<Response, Error>;

#[tokio::main]
async fn main() {
    // unsafe { backtrace_on_stack_overflow::enable() }
    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Cannot connect to Postgres");

    let origin = var("ORIGIN").expect("ORIGIN must be in .env");

    let routes = Router::new()
        .merge(UserController::routes())
        .merge(AuthController::routes())
        .merge(FolderController::routes())
        .merge(FileController::routes())
        .with_state(Arc::new(client))
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_origin(
                    origin
                        .parse::<HeaderValue>()
                        .expect("Failed to parse origin as HeaderValue"),
                )
                .allow_headers([
                    ORIGIN,
                    CONTENT_TYPE,
                    ACCEPT,
                    ACCESS_CONTROL_ALLOW_ORIGIN,
                    ACCESS_CONTROL_ALLOW_METHODS,
                    ACCESS_CONTROL_ALLOW_HEADERS,
                ])
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ]),
        );

    let port = var("PORT")
        .expect("Cannot read the PORT in the env")
        .parse()
        .expect("Cannot convert PORT variable into u16");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("Server crashed")
}
