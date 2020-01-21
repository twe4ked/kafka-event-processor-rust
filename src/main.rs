fn main() {
    let broker = std::env::var("KAFKA_BROKER").expect("KAFKA_BROKER not provided");
    let topic = std::env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC not provided");

    event_processor::run(broker, topic);
}
