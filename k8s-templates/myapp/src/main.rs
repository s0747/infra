use axum::{Router, http::StatusCode, routing::get};
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let app = Router::new()
        .route("/live", get(liveness_handler))
        .route("/ready", get(readiness_handler))
        .route("/healthz", get(healthz_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Serwer na http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn liveness_handler() -> StatusCode {
    StatusCode::OK
}

async fn readiness_handler() -> StatusCode {
    check_database_health()
}

async fn healthz_handler() -> StatusCode {
    if check_database_health() == StatusCode::OK {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

fn check_database_health() -> StatusCode {
    match env::var("DATABASE_URL") {
        Ok(url) if !url.is_empty() => StatusCode::OK,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    }
}
