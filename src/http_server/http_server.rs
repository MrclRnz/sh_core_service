use async_std::sync::{Arc, Mutex};
use tide::prelude::*;
use tide::Body;
use tide::Request;
use tide::Response;
use tide::StatusCode;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum DeviceType {
    Led,
    Lamp,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum State {
    On,
    Off,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Device {
    id: usize,
    dev_type: DeviceType,
    state: State,
}

#[derive(Clone, Default)]
struct Devices {
    devices: Arc<Mutex<Vec<Device>>>,
}

pub async fn start_server() -> tide::Result<()> {
    let mut app = tide::with_state(Devices::default());

    app.at("/devices").get(list_devices);
    app.at("/devices").post(register_device);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn list_devices(req: Request<Devices>) -> tide::Result<Response> {
    let devices = req.state().devices.lock().await;
    if let Ok(json) = serde_json::to_string::<Vec<Device>>(devices.as_ref()) {
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

async fn register_device(mut req: Request<Devices>) -> tide::Result<Response> {
    if let Ok(device) = req.body_json::<Device>().await {
        let mut devices = req.state().devices.lock_arc().await;
        devices.push(device);
        Ok(Response::new(StatusCode::Ok))
    } else {
        return Err(tide::Error::from_str(
            StatusCode::InternalServerError,
            "Failed to serialize json!",
        ));
    }
}
