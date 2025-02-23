// filepath: /rust-kafka-app/rust-kafka-app/src/main.rs
use std::time::Duration;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::error::KafkaError;
use futures::stream::StreamExt;

mod producer;
mod consumer;

#[tokio::main]
async fn main() -> Result<(), KafkaError> {
    let kafka_broker = "localhost:9092";
    let topic_name = "test_topic";

    // Initialize producer
    let producer = producer::Producer::new(kafka_broker).await?;
    
    // Initialize consumer
    let consumer = consumer::Consumer::new(kafka_broker, topic_name).await?;

    // Start message processing loop
    let consumer_stream = consumer.start().await?;
    
    tokio::spawn(async move {
        while let Some(message) = consumer_stream.next().await {
            match message {
                Ok(msg) => println!("Received message: {:?}", msg),
                Err(e) => eprintln!("Error receiving message: {:?}", e),
            }
        }
    });

    // Send messages to the topic
    for i in 0..10 {
        let message = format!("Message {}", i);
        producer.send_message(topic_name, message).await?;
    }

    // Allow some time for messages to be processed
    tokio::time::sleep(Duration::from_secs(5)).await;

    Ok(())
}