use dotenv::dotenv;
use mqtt_async_subscriber::{AsyncMqttSubscriber, MqttConfig};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().map_err(|e| format!("Error loading .env file: {}", e))?;

    let mqtt_config = MqttConfig {
        server_uri: env::var("MQTT_HOST")?,
        client_id: env::var("CLIENT_ID")?,
        trust_ca: env::var("TRUST_CF")?,
        user_name: env::var("MQTT_USER")?,
        password: env::var("MQTT_PASS")?,
    };

    let mut mqtt_client = AsyncMqttSubscriber::new(mqtt_config).await?;

    let topics = vec!["jack_sparrow", "davy_jones"];
    let qos_for_topic = vec![1, 0];
    // Add the topics you want to subscribe to

    mqtt_client
        .subscribe_to_topics(&topics, &qos_for_topic)
        .await?;

    mqtt_client.process_messages().await?;

    Ok(())
}
