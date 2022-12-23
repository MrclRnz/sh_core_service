use async_std::sync::{Arc, Mutex};
use rumqttc::Client;
use rumqttc::Connection;

use crate::http_server::controllers::devices::list_devices;
use crate::http_server::controllers::devices::register_device;
use crate::http_server::controllers::mqtt::is_alive;

use super::controllers::devices::Device;

pub struct HttpServerState {
    pub devices: Vec<Device>,
    pub mqtt_client: Client,
    pub mqtt_connection: Connection,
}

pub async fn start_server(init_state: Arc<Mutex<HttpServerState>>) -> tide::Result<()> {
    println!("Starting HTTP Server!");
    let mut app = tide::with_state(init_state);

    app.at("/devices").get(list_devices);
    app.at("/devices").post(register_device);

    app.at("/mqtt/health").get(is_alive);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
