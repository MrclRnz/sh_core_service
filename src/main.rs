use std::sync::Arc;

use async_std::sync::Mutex;
use sh_core_service::{
    http_server::http_server::{self, HttpServerState},
    mqtt_client,
};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let (mqtt_client, mqtt_connection) = mqtt_client::create_connection();
    let init_state = Arc::new(Mutex::new(HttpServerState {
        devices: Vec::new(),
        mqtt_client,
        mqtt_connection,
    }));

    http_server::start_server(init_state)
        .await
        .expect("Could not start HTTP server!");

    Ok(())
}
