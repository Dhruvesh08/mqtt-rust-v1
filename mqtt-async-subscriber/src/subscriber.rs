use futures_util::StreamExt;
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, SslOptionsBuilder};
use serde::{Deserialize, Serialize};
use std::{error::Error, process::Command};

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

#[derive(Debug, Serialize, Deserialize)]
struct LedResponse {
    led: String,
}

fn execute_command(command: &str) -> Result<(), std::io::Error> {
    let output = Command::new("/bin/sh").arg("-c").arg(command).output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Command '{}' failed with exit code {:?}",
                command,
                output.status.code()
            ),
        ))
    }
}

fn handle_mqtt_response(response: &str) {
    let led_response: LedResponse = serde_json::from_str(response).unwrap();
    match led_response.led.as_str() {
        "default-on" => {
            // Turn LED on
            println!("Turning LED on...");
            execute_command("echo default-on | sudo tee /sys/class/leds/sys_led/trigger").unwrap()
        }
        "none" => {
            // Turn LED off
            println!("Turning LED off...");
            execute_command("echo none | sudo tee /sys/class/leds/sys_led/trigger").unwrap()
        }
        "heartbeat" => {
            // Turn LED off
            println!("Turning LED Blink...");
            execute_command("echo heartbeat | sudo tee /sys/class/leds/sys_led/trigger").unwrap()
        }
        _ => println!("Invalid input!"),
    }
}

impl AsyncMqttSubscriber {
    pub fn new(config: &MqttConfig) -> Result<Self, Box<dyn Error>> {
        let create_opts = CreateOptionsBuilder::new()
            .server_uri(&config.server_uri)
            .client_id(&config.client_id)
            .finalize();

        let client = AsyncClient::new(create_opts)?;

        Ok(Self { client })
    }

    pub async fn connect(&mut self, config: MqttConfig) -> Result<(), Box<dyn Error>> {
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

        self.client.connect(conn_opts).await?;

        println!("Connected to broker");

        Ok(())
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
                handle_mqtt_response(&msg.payload_str());
            } else {
                break;
            }
        }

        Ok(())
    }

    pub async fn disconnect(self) -> Result<(), Box<dyn Error>> {
        self.client.disconnect(None).await?;

        println!("Disconnected from broker");

        Ok(())
    }
}
