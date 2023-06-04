use futures_util::StreamExt;
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, SslOptionsBuilder};
use std::error::Error;

pub struct MqttConfig {
    pub server_uri: String,
    pub client_id: String,
    pub trust_ca: String,
    pub user_name: String,
    pub password: String,
}

pub struct AsyncMqttSubscriber {
    client: AsyncClient,
}

impl AsyncMqttSubscriber {
    pub async fn new(config: MqttConfig) -> Result<Self, Box<dyn Error>> {
        let create_opts = CreateOptionsBuilder::new()
            .server_uri(config.server_uri)
            .client_id(config.client_id)
            .finalize();

        let client = AsyncClient::new(create_opts)?;

        let conn_opts = ConnectOptionsBuilder::new()
            .keep_alive_interval(std::time::Duration::from_secs(20))
            .clean_session(true)
            .user_name(config.user_name)
            .password(config.password)
            .ssl_options(
                SslOptionsBuilder::new()
                    .trust_store(config.trust_ca)?
                    .finalize(),
            )
            .finalize();

        client.connect(conn_opts).await?;

        println!("Connected to broker");

        Ok(Self { client })
    }

    pub async fn subscribe_to_topics(
        &self,
        topics: &[&str],
        qos: &[i32],
    ) -> Result<(), Box<dyn Error>> {
        let topic_filters: Vec<_> = topics.iter().map(|&topic| topic.to_string()).collect();

        self.client.subscribe_many(&topic_filters, &qos).await?;
        Ok(())
    }

    pub async fn process_messages(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stream = self.client.get_stream(10);
        while let Some(msg) = stream.next().await {
            if let Some(msg) = msg {
                println!(
                    "Received message on topic {}: {:?}",
                    msg.topic(),
                    msg.payload_str()
                );
            } else {
                break;
            }
        }

        Ok(())
    }
}
