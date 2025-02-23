# Simple Apache Kafka with Copilot

## First prompt

```
I'm trying to learn Apache Kafka can you create me a simple application using Rust as the provider and consumer
```

This did create a Rust app with a consumer and a producer but didn't create a Kafka instance, e.g. via `docker-compose`

I thought I'd run the `cargo run --bin producer` command anyway to see the result, which was:

```
error: no bin target named composer
```

I thought at this point I'd try Cursor.
