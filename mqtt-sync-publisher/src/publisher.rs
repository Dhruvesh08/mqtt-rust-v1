use paho_mqtt as mqtt;
use thiserror::Error;

pub struct MqttClient {
    url: String,
    client_id: String,
    topic: String,
    qos: i32,
    payload: String,
}

#[derive(Error, Debug)]
pub enum MqttError {
    #[error("Failed to connect to MQTT broker: {0}")]
    ConnectError(#[from] mqtt::Error),
    #[error("Failed to publish message: {0}")]
    PublishError(mqtt::Error),
}

impl MqttClient {
    pub fn new(
        url: String,
        client_id: String,
        topic: String,
        qos: i32,
        payload: String,
    ) -> MqttClient {
        MqttClient {
            url,
            client_id,
            topic,
            qos,
            payload,
        }
    }

    pub fn connect(&self) -> Result<mqtt::Client, MqttError> {
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(&self.url)
            .client_id(&self.client_id)
            .finalize();

        let client = mqtt::Client::new(create_opts)?;

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(std::time::Duration::from_secs(30))
            .clean_session(true)
            .finalize();

        match client.connect(conn_opts) {
            Ok(_) => Ok(client),
            Err(e) => Err(MqttError::ConnectError(e)),
        }
    }

    pub fn publish(&self, client: &mqtt::Client) -> Result<(), MqttError> {
        let msg = mqtt::MessageBuilder::new()
            .topic(&self.topic)
            .payload(&*self.payload)
            .qos(self.qos)
            .finalize();

        match client.publish(msg) {
            Ok(_) => Ok(()),
            Err(e) => Err(MqttError::PublishError(e)),
        }
    }
}
