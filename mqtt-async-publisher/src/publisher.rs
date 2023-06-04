use mqtt::{CreateOptionsBuilder, SslOptionsBuilder};
use paho_mqtt as mqtt;
use std::error::Error;

pub struct MqttConfig {
    pub server_uri: String,
    pub client_id: String,
    pub trust_ca: String,
    pub user_name: String,
    pub password: String,
}

pub struct AsyncMqttPublisher {
    client: mqtt::AsyncClient,
}

impl AsyncMqttPublisher {
    pub fn new(config: &MqttConfig) -> Result<Self, Box<dyn Error>> {
        let create_opts = CreateOptionsBuilder::new()
            .server_uri(&config.server_uri)
            .client_id(&config.client_id)
            .finalize();

        let client = mqtt::AsyncClient::new(create_opts)?;

        // return client or error
        Ok(Self { client })
    }

    pub async fn connect(&self, config: &MqttConfig) -> Result<(), mqtt::Error> {
        let conn_opts = mqtt::ConnectOptionsBuilder::new_v5()
            .clean_session(true)
            .user_name(&config.user_name)
            .password(&config.password)
            .ssl_options(
                SslOptionsBuilder::new()
                    .trust_store(&config.trust_ca)?
                    .finalize(),
            )
            .finalize();

        println!("Connecting to the MQTT server");
        self.client.connect(conn_opts).await?;

        Ok(())
    }

    pub async fn publish(&self, topic: &str, message: &str) -> Result<(), mqtt::Error> {
        // Create a message and publish it
        println!("Publishing a message on the topic '{}'", topic);
        let msg = mqtt::Message::new(topic, message, mqtt::QOS_1);
        self.client.publish(msg).await?;

        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), mqtt::Error> {
        // Disconnect from the broker
        println!("Disconnecting");
        self.client.disconnect(None).await?;

        Ok(())
    }
}
