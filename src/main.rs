use std::{net::SocketAddr, sync::Arc};

use axum::response::Response;
use axum_server::tls_rustls::RustlsConfig;
use dotenvy::dotenv;
use tokio::net::TcpListener;

use aws::S3;
use config::{check_env, hostname, https, port, setup_cors, ssl_cert_key};
use error::Error;
use file::service::FileService;
use file_version::service::FileVersionService;
use folder::service::FolderService;
use prisma::PrismaClient;
use routes::routes;
use tag::service::TagService;
use user::service::UserService;

mod auth;
mod aws;
mod config;
mod error;
mod extractors;
mod file;
mod file_version;
mod folder;
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
	pub user_service: UserService,
	pub folder_service: FolderService,
	pub file_service: FileService,
	pub file_version_service: FileVersionService,
	pub tag_service: TagService,
	pub storage: S3,
}

type WebResult = Result<Response, Error>;

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
		user_service: UserService::init(&client),
		folder_service: FolderService::init(&client),
		file_service: FileService::init(&client),
		file_version_service: FileVersionService::init(&client),
		tag_service: TagService::init(&client),
		storage: S3::init(),
	};

	let routes = routes().with_state(state).layer(setup_cors());

	let hostname = hostname();
	let port = port();

	if https() {
		let tls_config = {
			let (cert_path, key_path) = ssl_cert_key();
			RustlsConfig::from_pem_file(cert_path, key_path)
				.await
				.expect("Cannot find certifications to enable https")
		};

		let addr = SocketAddr::from(([0, 0, 0, 0], port));

		println!("Server started at https://{hostname}:{port}");

		axum_server::bind_rustls(addr, tls_config)
			.serve(routes.into_make_service())
			.await
			.expect("Server crashed")
	} else {
		let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

		println!("Server started at http://{hostname}:{port}");

		axum::serve(listener, routes).await.expect("Server crashed");
	}
}
