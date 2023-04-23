use axum::{extract::State, response::Json, routing::get, Router};
use eyre::WrapErr as _;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::relayer::State as RelayerState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthResponse {
    pub status: String,
}

async fn handle_healthz(State(state): State<Arc<Mutex<RelayerState>>>) -> Json<HealthResponse> {
    let state = state.lock().await;
    let status = if state.curr_sequencer_height.is_none() || state.curr_da_height.is_none() {
        "degraded"
    } else {
        "ok"
    };

    Json(HealthResponse {
        status: status.to_string(),
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusResponse {
    pub curr_sequencer_height: Option<u64>,
    pub curr_da_height: Option<u64>,
}

async fn handle_status(State(state): State<Arc<Mutex<RelayerState>>>) -> Json<StatusResponse> {
    let state = state.lock().await;
    Json(StatusResponse {
        curr_sequencer_height: state.curr_sequencer_height,
        curr_da_height: state.curr_da_height,
    })
}

pub async fn start(
    listen_url: &str,
    listen_port: u16,
    relayer: Arc<Mutex<RelayerState>>,
) -> eyre::Result<()> {
    let app = Router::new()
        .route("/healthz", get(handle_healthz))
        .route("/status", get(handle_status))
        .with_state(relayer);

    let listen_addr = format!("{}:{}", listen_url, listen_port);

    axum::Server::bind(
        &listen_addr
            .parse()
            .wrap_err("failed to parse listen_addr")?,
    )
    .serve(app.into_make_service())
    .await
    .wrap_err("failed to serve")?;

    Ok(())
}
