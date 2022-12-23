use async_std::sync::Mutex;
use std::sync::Arc;
use tide::{Body, Request, Response, StatusCode};

use crate::http_server::http_server::HttpServerState;

// GET /mqtt?type=health
pub async fn is_alive(req: Request<Arc<Mutex<HttpServerState>>>) -> tide::Result<Response> {
    let state_reference_mutex = Arc::clone(req.state());
    let mut state_guard = state_reference_mutex.lock().await;

    // Iterate to poll the eventloop for connection progress
    if let Some(notification) = state_guard.mqtt_connection.iter().next() {
        let is_alive = match notification {
            Ok(_) => true,
            Err(_) => false,
        };
        return Ok(Response::builder(StatusCode::Ok)
            .body(Body::from_string(format!("{{\"alive\": {}}}", is_alive)))
            .build());
    } else {
        return Err(tide::Error::from_str(
            StatusCode::InternalServerError,
            "No connection health information could be retrieved!",
        ));
    }
}

//client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();
//client.publish("hello/rumqtt", QoS::AtLeastOnce, false, "test").unwrap();
