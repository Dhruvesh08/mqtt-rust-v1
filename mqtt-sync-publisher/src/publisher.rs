use paho_mqtt as mqtt;

pub struct MqttClient {
    url: String,
    client_id: String,
    topic: String,
    qos: i32,
    payload: String,
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

    pub fn connect(&self) -> Result<mqtt::Client, mqtt::Error> {
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(&self.url)
            .client_id(&self.client_id)
            .finalize();

        let client = mqtt::Client::new(create_opts)?;

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(std::time::Duration::from_secs(30))
            .clean_session(true)
            .finalize();

        client.connect(conn_opts)?;

        Ok(client)
    }

    pub fn publish(&self, client: &mqtt::Client) -> Result<(), mqtt::Error> {
        let msg = mqtt::MessageBuilder::new()
            .topic(&self.topic)
            .payload(&*self.payload)
            .qos(self.qos)
            .finalize();

        client.publish(msg)?;

        Ok(())
    }
}
