#![allow(dead_code, unused_variables)]

use std::{net::SocketAddr, sync::Arc};

use auth::controller::auth_routes;
use aws::S3;
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
use file::controller::file_routes;
use folder::controller::folder_routes;
use prisma::PrismaClient;

use tower_http::cors::CorsLayer;
use user::controller::user_routes;

mod auth;
mod aws;
mod error;
mod extractors;
mod folder;

mod file;
#[allow(warnings)]
mod prisma;
mod user;
mod validation;
mod web;

#[derive(Clone)]
pub struct GlobalState {
    pub storage: S3,
    pub db: Arc<PrismaClient>,
}

type WebResult = std::result::Result<Response, Error>;

#[tokio::main]
async fn main() {
    // unsafe { backtrace_on_stack_overflow::enable() }
    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Cannot connect to Postgres");

    let state = GlobalState {
        storage: S3::init(),
        db: Arc::new(client),
    };

    let origin = var("ORIGIN").expect("ORIGIN must be in .env");

    let routes = Router::new()
        .merge(user_routes())
        .merge(auth_routes())
        .merge(folder_routes())
        .merge(file_routes())
        .with_state(state)
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
