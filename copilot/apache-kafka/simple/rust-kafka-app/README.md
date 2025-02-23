# Simple Apache Kafka Rust Application

This project implements a simple Apache Kafka producer and consumer application in Rust. It demonstrates how to send and receive messages using Kafka.

## Project Structure

```
rust-kafka-app
├── src
│   ├── main.rs        # Entry point of the application
│   ├── producer.rs    # Producer implementation
│   └── consumer.rs    # Consumer implementation
├── Cargo.toml         # Project configuration
└── README.md          # Project documentation
```

## Setup Instructions

1. Ensure you have Rust installed on your machine. You can install it from [rustup.rs](https://rustup.rs/).
2. Clone this repository or download the project files.
3. Navigate to the project directory:
   ```
   cd rust-kafka-app
   ```
4. Add the necessary dependencies to your `Cargo.toml` file. You will need a Kafka client library, such as `rdkafka`.

## Usage

### Running the Producer

To run the producer, execute the following command:

```
cargo run --bin producer
```

### Running the Consumer

To run the consumer, execute the following command:

```
cargo run --bin consumer
```

## Producer Functionality

The producer is responsible for sending messages to a specified Kafka topic. It initializes a Kafka producer instance and provides methods to send messages.

## Consumer Functionality

The consumer listens to a specified Kafka topic and processes incoming messages. It initializes a Kafka consumer instance and provides methods to handle message consumption.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.