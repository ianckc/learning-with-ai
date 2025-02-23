// filepath: /rust-kafka-app/rust-kafka-app/src/consumer.rs
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::message::Message;
use rdkafka::ClientConfig;

pub struct KafkaConsumer {
    consumer: BaseConsumer,
}

impl KafkaConsumer {
    pub fn new(brokers: &str, group_id: &str, topic: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let consumer: BaseConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", group_id)
            .set("auto.offset.reset", "earliest")
            .create()?;

        consumer.subscribe(&[topic])?;

        Ok(KafkaConsumer { consumer })
    }

    pub fn consume(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match self.consumer.recv() {
                Ok(message) => {
                    if let Some(payload) = message.payload() {
                        println!("Received message: {:?}", std::str::from_utf8(payload)?);
                    }
                }
                Err(e) => {
                    eprintln!("Error while receiving message: {:?}", e);
                }
            }
        }
    }
}