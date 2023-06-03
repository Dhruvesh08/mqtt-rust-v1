use dotenv::dotenv;
use paho_mqtt as mqtt;
use thiserror::Error;

pub struct MqttSubscriber {
    pub client: mqtt::Client,
    topics: Vec<String>,
}

#[derive(Error, Debug)]
pub enum MqttError {
    #[error("Failed to connect to MQTT broker: {0}")]
    ConnectError(#[from] mqtt::Error),
    #[error("Failed to publish message: {0}")]
    PublishError(mqtt::Error),
}

impl MqttSubscriber {
    pub fn new(server_uri: &str, client_id: &str) -> Self {
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(server_uri)
            .client_id(client_id)
            .finalize();

        let client = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
            println!("Error creating the client: {:?}", err);
            std::process::exit(1);
        });

        Self {
            client,
            topics: Vec::new(),
        }
    }

    pub fn connect(&self) -> Result<(), mqtt::Error> {
        dotenv().ok();

        let trust_ca = std::env::var("TRUST_CF").unwrap();
        println!("Connecting to {} in subscriber", trust_ca);
        let user_name = std::env::var("MQTT_USER").unwrap();
        let password = std::env::var("MQTT_PASS").unwrap();

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(std::time::Duration::from_secs(20))
            .clean_session(true)
            .user_name(&user_name)
            .password(&password)
            .ssl_options(
                mqtt::SslOptionsBuilder::new()
                    .trust_store(&trust_ca)?
                    .finalize(),
            )
            .finalize();

        match self.client.connect(conn_opts) {
            Ok(_) => {
                println!("Connected to broker");
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn subscribe(&mut self, topics: Vec<&str>, qos: i32) -> Result<(), mqtt::Error> {
        for topic in topics {
            self.topics.push(topic.to_string());
            self.client.subscribe(topic, qos)?;
        }

        Ok(())
    }

    pub fn disconnect(&self) -> Result<(), mqtt::Error> {
        self.client.disconnect(None)
    }
}
