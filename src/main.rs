use dotenv::dotenv;

use mqtt_async_publisher::{AsyncMqttPublisher, MqttConfig};

use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().map_err(|e| format!("Error loading .env file: {}", e))?;

    let config = MqttConfig {
        server_uri: env::var("MQTT_HOST")?,
        client_id: env::var("CLIENT_ID")?,
        trust_ca: env::var("TRUST_CF")?,
        user_name: env::var("MQTT_USER")?,
        password: env::var("MQTT_PASS")?,
    };

    let publisher = AsyncMqttPublisher::new(&config)?;
    publisher.connect(&config).await?;

    let topic = "jack_sparrow/";
    let message = "Hello, world! form captain jack sparrow";
    publisher.publish(topic, message).await?;

    // publisher.disconnect().await?;

    Ok(())
}
