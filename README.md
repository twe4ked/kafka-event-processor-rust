# Event Processing

This repo contains two crates, `framework` and `app`.

The framework crate is responsible for connecting to a Kafka stream and parsing
the events into JSON. It exposes a simple `Processor<E>` trait that it then
passes the parsed events to.

The app crate implements a `Processor<E>` to handle the events where the `E`
type implements `From<serde_json::value::Value>`. The from implementation is
used to parse the JSON events into proper `Event`s to be processed.

### Example

```
$ KAFKA_BROKER=localhost:9092 KAFKA_TOPIC=survey-events-v2 RUST_LOG="app=INFO" cargo run
Jan 23 10:53:51.702  INFO process{}: app: SurveyCreatedBody {
    account_id: be06b40e-b423-40a4-9c03-f88622544b86,
    created_at: "2019-12-10 22:17:44 UTC",
    name: [
        LocalizedText {
            locale: "en",
            text: "Custom Survey Dec 2019 v9",
        },
    ],
}
```
