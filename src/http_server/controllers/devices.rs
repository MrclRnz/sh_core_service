use std::sync::Arc;

use async_std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response, StatusCode};
use ts_rs::TS;

use crate::http_server::http_server::HttpServerState;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
enum DeviceType {
    Led,
    Lamp,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
enum State {
    On,
    Off,
}
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Device {
    id: usize,
    dev_type: DeviceType,
    state: State,
}

// GET /devices
pub async fn list_devices(req: Request<Arc<Mutex<HttpServerState>>>) -> tide::Result<Response> {
    let devices = &req.state().lock().await.devices;
    if let Ok(json) = serde_json::to_string::<Vec<Device>>(devices) {
        let response = Response::builder(StatusCode::Ok)
            .body(Body::from_string(json))
            .build();
        Ok(response)
    } else {
        return Err(tide::Error::from_str(
            StatusCode::InternalServerError,
            "Failed to serialize json!",
        ));
    }
}

// POST /devices
pub async fn register_device(
    mut req: Request<Arc<Mutex<HttpServerState>>>,
) -> tide::Result<Response> {
    if let Ok(device) = req.body_json::<Device>().await {
        let state_reference_mutex = Arc::clone(req.state());
        let mut state_guard = state_reference_mutex.lock().await;
        state_guard.devices.push(device);
        Ok(Response::new(StatusCode::Ok))
    } else {
        return Err(tide::Error::from_str(
            StatusCode::InternalServerError,
            "Failed to serialize json!",
        ));
    }
}
