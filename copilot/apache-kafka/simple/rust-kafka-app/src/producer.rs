// filepath: /rust-kafka-app/rust-kafka-app/src/producer.rs
use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord};
use std::time::Duration;

pub struct Producer {
    producer: BaseProducer,
}

impl Producer {
    pub fn new(brokers: &str) -> Self {
        let producer: BaseProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()
            .expect("Producer creation failed");
        
        Producer { producer }
    }

    pub fn send_message(&self, topic: &str, key: &str, value: &str) {
        self.producer.send(
            BaseRecord::to(topic)
                .key(key)
                .payload(value),
        ).expect("Failed to enqueue message");
        
        // Wait for the producer to flush messages
        self.producer.flush(Duration::from_secs(1));
    }
}