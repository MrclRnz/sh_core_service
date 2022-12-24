use std::{sync::Arc};

use async_std::{sync::Mutex};
use sh_core_service::{
    http_server::http_server::{self, HttpServerState}, mqtt::mqtt_client
};

#[async_std::main]
async fn main() {
    let (mqtt_publisher_client, mut mqtt_connection) = mqtt_client::create_connection("frontend_client");
    let initial_health = if let Some(res) = mqtt_connection.iter().next() {
        res.is_ok()
    } else {
        false
    };
    let init_state = Arc::new(Mutex::new(HttpServerState {
        devices: Vec::new(),
        mqtt_client: mqtt_publisher_client,
        mqtt_alive: initial_health,
    }));

    http_server::manage_connection_health(Arc::clone(&init_state), mqtt_connection);
    http_server::start_server(init_state)
        .await
        .expect("Could not start HTTP server!");
}