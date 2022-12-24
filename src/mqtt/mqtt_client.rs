use rumqttc::{Client, Connection, MqttOptions};
use std::time::Duration;

pub fn create_connection(id: &str) -> (Client, Connection) {
    let mut mqttoptions = MqttOptions::new(id, "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    Client::new(mqttoptions, 10)
}

