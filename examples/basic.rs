struct Processor {}

impl event_processor::EventProcessor for Processor {
    fn process(&self, value: serde_json::Value) {
        dbg!(value);
    }
}

fn main() {
    let broker = std::env::var("KAFKA_BROKER").expect("KAFKA_BROKER not provided");
    let topic = std::env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC not provided");
    let event_processor = Processor {};

    event_processor::run(broker, topic, event_processor);
}
