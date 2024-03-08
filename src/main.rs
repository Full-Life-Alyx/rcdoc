#![allow(unused_imports)]

use std::fs::File;

use const_format::concatcp;
use dotenv_codegen::dotenv;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, EndpointExt, Route};
use poem_grants::GrantsMiddleware;
use poem_openapi::OpenApiService;
use sqlx::PgPool;
use time::OffsetDateTime;

use crate::auth::middleware::auth_extractor;

const PORT: u16 = 80;
pub const DOMAIN: &str = "localhost";

pub mod api;
pub mod auth;
pub mod store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    color_eyre::install().unwrap();
    // Logging setup
    let tracing_sub = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(tracing_sub).unwrap();
    dotenv::dotenv().ok();

    // Init database and migrations
    const DATABASE_URL: &str = env!("PROD_DATABASE_URL", "PROD_DATABASE_URL must be passed in");
    let pool = PgPool::connect(DATABASE_URL).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let client = redis::Client::open("redis://redis:6379").unwrap();
    let connection = client.get_multiplexed_tokio_connection().await.unwrap();

    /*
    // Init services
    let main_service = OpenApiService::new(MainApi::new(pool.clone()), "Main", "1.0.0")
        .server(concatcp!("http://localhost:", PORT, "/api"));
    let auth_service = OpenApiService::new(AuthApi::new(pool.clone()), "Auth", "1.0.0")
        .server(concatcp!("http://localhost:", PORT, "/api/oauth"));
    let thread_service = OpenApiService::new(ThreadApi::new(pool.clone()), "Thread", "1.0.0")
        .server(concatcp!("http://localhost:", PORT, "/api/thread"));
        */

    let app = todo!();
    /*
     Route::new()
        .nest(
            "/",
            StaticFilesEndpoint::new("dist/")
                .show_files_listing()
                .index_file("index.html")
                .with(GrantsMiddleware::with_extractor(auth_extractor))
        )
        .nest(
            "/api",
            Route::new()
                .nest("/docs", main_service.swagger_ui())
                .nest("/", main_service)
                .nest("/oauth/docs", auth_service.swagger_ui())
                .nest("/oauth", auth_service)
                .nest("/forum/docs", thread_service.swagger_ui())
                .nest("/forum", thread_service)
                .with(GrantsMiddleware::with_extractor(auth_extractor)),
        );

    let listener = TcpListener::bind(("0.0.0.0", PORT));
    println!(
        "Starting server on port {} (http://localhost:{})",
        PORT, PORT
    );
    poem::Server::new(listener).run(app).await?;
        */

    Ok(())
}
