use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Incoming};
use tokio::time::{Duration};
use chrono::Local;
use colored::*;

#[tokio::main]
async fn main() {
    // MQTT client configuration
    let mut mqttoptions = MqttOptions::new("mqtt-rust-client", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Subscribe to RX and TX topics
    let rx_topic = "pza/serial plugin/serial-stream/RX/att";
    let tx_topic = "pza/serial plugin/serial-stream/TX/cmd";
    client.subscribe(rx_topic, QoS::AtMostOnce).await.unwrap();
    client.subscribe(tx_topic, QoS::AtMostOnce).await.unwrap();

    println!("{}", "🛰️  Starting MQTT observer...".bold().cyan());

    // Listening loop
    loop {
        let event = eventloop.poll().await;

        match event {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                let topic = p.topic;
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let payload_bytes = p.payload;

                // Converts bytes into an escaped String
                let escaped_payload = payload_bytes
                    .iter()
                    .flat_map(|b| std::ascii::escape_default(*b))
                    .collect::<Vec<u8>>();

                let payload_str = String::from_utf8(escaped_payload)
                    .unwrap_or_else(|_| "<non UTF-8>".to_string());
                
                match topic.as_str() {
                    t if t==rx_topic => {
                        println!(
                            "{} [{}] {}: {:?}",
                            "📥".green(),
                            timestamp,
                            "RX".bold().green(),
                            payload_str
                        );
                    }
                    t if t==tx_topic => {
                        println!(
                            "{} [{}] {}: {:?}",
                            "📤".blue(),
                            timestamp,
                            "TX".bold().blue(),
                            payload_str
                        );
                    }
                    _ => {
                        println!(
                            "{} [{}] {}: {:?}",
                            "❓".yellow(),
                            timestamp,
                            topic.bold().yellow(),
                            payload_str
                        );
                    }
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("{} Error in event loop: {}", "❌".red(), e);
                break;
            }
        }
    }
}