use async_std::sync::Mutex;
use rumqttc::QoS;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tide::{Body, Request, Response, StatusCode};
use ts_rs::TS;

use crate::http_server::http_server::HttpServerState;

#[derive(Serialize, TS)]
#[ts(export)]
struct MqttHealth {
    alive: bool,
}


#[derive(Deserialize, TS)]
#[ts(export)]
struct MqttCoreMessage {
    message: String,
}

// GET /mqtt?type=health
pub async fn is_alive(req: Request<Arc<Mutex<HttpServerState>>>) -> tide::Result<Response> {
    let state_reference_mutex = Arc::clone(req.state());
    let state_guard = state_reference_mutex.lock().await;

    if let Ok(json) = serde_json::to_string(&MqttHealth {
        alive: state_guard.mqtt_alive,
    }) {
        return Ok(Response::builder(StatusCode::Ok)
            .body(Body::from_string(json))
            .build());
    }
    return Err(tide::Error::from_str(
        StatusCode::InternalServerError,
        "No connection health information could be retrieved!",
    ));
}

// POST /mqtt
pub async fn publish_message(mut req: Request<Arc<Mutex<HttpServerState>>>) -> tide::Result<Response> {
    if let Ok(message) = req.body_json::<MqttCoreMessage>().await {
        let state_reference_mutex = Arc::clone(req.state());
        let mut state_guard = state_reference_mutex.lock().await;
        state_guard.mqtt_client.publish("smart_home/core_service", QoS::AtLeastOnce, false, message.message).unwrap();
        Ok(Response::new(StatusCode::Ok))
    } else {
        return Err(tide::Error::from_str(
            StatusCode::InternalServerError,
            "Failed to serialize json!",
        ));
    }
}