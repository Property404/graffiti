mod api;
mod errors;
mod model;
mod routes;

use axum::{routing::get_service, Router};
use clap::Parser;
use errors::{Error, Result};
use model::ModelController;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

/// Graffiti server. Draw with friends!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on.
    #[arg(long, default_value_t = 8080)]
    port: u16,
    /// Static public directory.
    #[arg(long)]
    public: Option<String>,
}

#[tokio::main]
async fn main() -> Result {
    let args = Args::parse();

    let mc = ModelController::default();

    let routes = Router::new().nest_service("/api", routes::routes(mc));
    let routes = if let Some(public_dir) = args.public {
        routes.merge(Router::new().nest_service("/", get_service(ServeDir::new(public_dir))))
    } else {
        routes
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await?;

    Ok(())
}
