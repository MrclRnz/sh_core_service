use rumqttc::QoS;
use sh_core_service::{http_server::http_server, mqtt_client};

#[async_std::main]
async fn main() -> tide::Result<()> {
    http_server::start_server().await?;
    //let mut client = mqtt_client::create_connection();
    //client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();
    //client.publish("hello/rumqtt", QoS::AtLeastOnce, false, "test").unwrap();



    Ok(())
}
