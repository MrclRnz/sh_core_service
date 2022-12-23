use rumqttc::{Client, Connection, MqttOptions};
use std::time::Duration;

pub fn create_connection() -> (Client, Connection) {
    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "test.mosquitto.org", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, connection) = Client::new(mqttoptions, 10);
    (client, connection)
}
