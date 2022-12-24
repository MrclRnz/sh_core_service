use std::thread;

use async_std::sync::{Arc, Mutex};
use async_std::task::block_on;
use rumqttc::{Client, Connection};

use crate::http_server::controllers::devices::list_devices;
use crate::http_server::controllers::devices::register_device;
use crate::http_server::controllers::mqtt::is_alive;
use crate::http_server::controllers::mqtt::publish_message;

use super::controllers::devices::Device;

pub struct HttpServerState {
    pub devices: Vec<Device>,
    pub mqtt_client: Client,
    pub mqtt_alive: bool,
}

pub async fn start_server(init_state: Arc<Mutex<HttpServerState>>) -> tide::Result<()> {
    println!("Starting HTTP Server!");
    let mut app = tide::with_state(init_state);

    app.at("/devices").get(list_devices);
    app.at("/devices").post(register_device);

    app.at("/mqtt/health").get(is_alive);
    app.at("/mqtt").post(publish_message);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

pub fn manage_connection_health(server_state: Arc<Mutex<HttpServerState>>, mut mqtt_connection: Connection) {
    thread::spawn(move || {
        block_on(async {
            let mut current_health = server_state.lock().await.mqtt_alive;
            for notification in mqtt_connection.iter() {
                if current_health != notification.is_ok() {
                    current_health = !current_health;
                    set_connection_health(&server_state, notification.is_ok()).await;
                }
            }
        });
    });
}

async fn set_connection_health(server_state: &Arc<Mutex<HttpServerState>>, health: bool) {
    let mut state_guard = server_state.lock().await;
    state_guard.mqtt_alive = health;
}