use mqtt_sync_publisher::publisher::MqttClient;

fn main() {
    let broker_url = "mqtt://broker.emqx.io:1883";
    let client_id = "mqtt-pub";
    let topic = "jack/";
    let payload = "Hello from Rust! this is cool code";
    let qos = 1;

    let mqtt_sync_subscriber = MqttClient::new(
        broker_url.to_string(),
        client_id.to_string(),
        topic.to_string(),
        qos,
        payload.to_string(),
    );
    // let client = mqtt_sync_subscriber.connect();

    let client = match mqtt_sync_subscriber.connect() {
        Ok(client) => client,
        Err(e) => {
            println!("Error creating the client: {:?}", e);
            std::process::exit(1);
        }
    };

    match mqtt_sync_subscriber.publish(&client) {
        Ok(_) => print!("Message published.\n"),
        Err(e) => {
            println!("Error sending message: {:?}", e);
            std::process::exit(1);
        }
    }
}
