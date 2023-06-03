extern crate dotenv;
use mqtt_sync_subscriber::MqttSubscriber;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let mqtt_server_uri = std::env::var("MQTT_HOST").unwrap();
    let client_id = std::env::var("CLIENT_ID").unwrap();

    print!(
        "Connecting to {} with client id {}",
        mqtt_server_uri, client_id
    );

    let mut mqtt_client = MqttSubscriber::new(&mqtt_server_uri.to_string(), &client_id.to_string());

    if let Err(e) = mqtt_client.connect() {
        println!("Unable to connect: {:?}", e);
        std::process::exit(1);
    }

    if let Err(e) = mqtt_client.subscribe(vec!["jack_sparrow", "will_turner"], 1) {
        println!("Error subscribing to topic: {:?}", e);
        std::process::exit(1);
    }

    for msg in mqtt_client.client.start_consuming() {
        if let Some(msg) = msg {
            println!(
                "Received message on topic {}: {:?}",
                msg.topic(),
                msg.payload_str()
            );
            // break;
        }
    }

    if let Err(e) = mqtt_client.disconnect() {
        println!("Error disconnecting from broker: {:?}", e);
        std::process::exit(1);
    }
}
