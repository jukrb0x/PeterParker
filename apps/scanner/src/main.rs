use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
    extract::{State, WebSocketUpgrade, Path},
    response::IntoResponse,
    Json,
};
use tower_http::cors::{CorsLayer, Any};
use tokio::sync::RwLock;

use peterparker_core::models::{Device, ScanProgress, ScanResult};

mod handlers;
mod ws;

#[derive(Clone)]
pub struct AppState {
    pub devices: Arc<RwLock<Vec<Device>>>,
    pub scans: Arc<RwLock<Vec<(ScanProgress, Option<ScanResult>)>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        devices: Arc::new(RwLock::new(Vec::new())),
        scans: Arc::new(RwLock::new(Vec::new())),
    };

    // Seed with mock data
    {
        let mut devices = state.devices.write().await;
        *devices = seed_mock_devices();
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/devices", get(handlers::get_devices))
        .route("/api/devices/:ip", get(handlers::get_device))
        .route("/api/scan", post(handlers::start_scan))
        .route("/api/scan/:id", get(handlers::get_scan_progress))
        .route("/ws", get(ws_handler))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("✓ Scanner service running on http://{}", addr);
    println!("✓ WebSocket endpoint: ws://{}/ws", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| ws::handle_socket(socket, state))
}

fn seed_mock_devices() -> Vec<Device> {
    use peterparker_core::models::{OperatingSystem, Port, Service, DeviceMetadata, DeviceType, Protocol, PortState};
    use chrono::Utc;
    use uuid::Uuid;

    vec![
        Device {
            id: Uuid::new_v4().to_string(),
            ip: "192.168.1.1".to_string(),
            mac: Some("00:11:22:33:44:55".to_string()),
            vendor: Some("TP-Link".to_string()),
            hostname: Some("Router".to_string()),
            os: Some(OperatingSystem {
                name: "Linux".to_string(),
                family: "linux".to_string(),
                version: None,
                confidence: 90,
                cpe: vec![],
            }),
            device_type: DeviceType::Router,
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            is_online: true,
            ports: vec![Port {
                number: 80,
                protocol: Protocol::Tcp,
                state: PortState::Open,
                service: Some(Service {
                    name: "http".to_string(),
                    version: None,
                    product: None,
                    extra_info: serde_json::json!({}),
                }),
                banner: None,
            }],
            metadata: DeviceMetadata {
                http_title: Some("Router".to_string()),
                http_server: Some("nginx".to_string()),
                ssh_version: None,
                smb_info: None,
                ttl: Some(64),
                window_size: None,
            },
        },
    ]
}
