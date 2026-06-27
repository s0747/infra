use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use lazy_static::lazy_static;
use prometheus::{
    register_histogram_vec, register_int_counter_vec, register_int_gauge, Encoder, HistogramVec,
    IntCounterVec, IntGauge, TextEncoder,
};
use serde::Serialize;
use std::env;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct RouterEndpoints {
    path: String,
    method: String,
    description: String,
}

#[derive(Serialize)]
struct HomeResponse {
    status: String,
    pod_name: String,
    available_urls: Vec<RouterEndpoints>,
}

lazy_static! {
    // 1. NOWA METRYKA: Aktualny czas Unix Timestamp (w sekundach)
    static ref APP_CURRENT_TIME: IntGauge = register_int_gauge!(
        "app_current_time_seconds",
        "Aktualny czas systemowy aplikacji jako Unix Timestamp."
    ).unwrap();

    // Poprzednie metryki
    static ref ORDER_COUNTER: IntCounterVec = register_int_counter_vec!(
        "shop_orders_total",
        "Calkowita liczba zlozonych zamowien w sklepie.",
        &["payment_method"]
    ).unwrap();

    static ref DB_QUERY_DURATION: HistogramVec = register_histogram_vec!(
        "db_query_duration_seconds",
        "Czas trwania zapytan do bazy danych w sekundach.",
        &["operation"],
        vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]
    ).unwrap();
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/live", get(liveness_handler))
        .route("/ready", get(readiness_handler))
        .route("/healthz", get(healthz_handler))
        .route("/metrics", get(metrics_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Serwer na http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// async fn home_handler() -> &'static str {
//     "Witaj w aplikacji!"
// }
// ---
// pub async fn home_handler() -> impl IntoResponse {
//     let pod_name = env::var("HOSTNAME").unwrap_or_else(|_| "unknown-pod".to_string());

//     let message = format!("Hello from Pod: {}\n", pod_name);

//     (StatusCode::OK, message)
// }
pub async fn home_handler() -> impl IntoResponse {
    // Pobieramy nazwę poda z K8s
    let pod_name = env::var("HOSTNAME").unwrap_or_else(|_| "local-machine".to_string());

    // Dokładna lista Twoich tras z kodu aplikacji:
    let available_urls = vec![
        RouterEndpoints {
            path: "/".to_string(),
            method: "GET".to_string(),
            description: "Strona główna z listą endpointów".to_string(),
        },
        RouterEndpoints {
            path: "/live".to_string(),
            method: "GET".to_string(),
            description: "K8s Liveness Probe".to_string(),
        },
        RouterEndpoints {
            path: "/ready".to_string(),
            method: "GET".to_string(),
            description: "K8s Readiness Probe".to_string(),
        },
        RouterEndpoints {
            path: "/healthz".to_string(),
            method: "GET".to_string(),
            description: "Ogólny status".to_string(),
        },
        RouterEndpoints {
            path: "/metrics".to_string(),
            method: "GET".to_string(),
            description: "Metryki Prometheus / OpenMetrics".to_string(),
        },
    ];

    let response = HomeResponse {
        status: "OK".to_string(),
        pod_name,
        available_urls,
    };

    (StatusCode::OK, Json(response))
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

// Handler dla /metrics
async fn metrics_handler() -> String {
    // POBIERANIE AKTUALNEGO CZASU UNIX i aktualizacja metryki przed wysłaniem
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Czas cofnal sie w przeszlosc")
        .as_secs();

    APP_CURRENT_TIME.set(now as i64);

    // Standardowe kodowanie metryk do OpenMetrics
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();

    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
