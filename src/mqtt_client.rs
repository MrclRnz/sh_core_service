use rumqttc::{Client, MqttOptions};
use std::time::Duration;

pub fn create_connection() -> Client {
    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "test.mosquitto.org", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(mqttoptions, 10);
    
    // Iterate to poll the eventloop for connection progress
    for (_i, notification) in connection.iter().enumerate() {
        println!("Notification = {:?}", notification);
    }
    client
}
