use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;

pub fn run<T>(broker: String, topic: String, event_processor: T)
where
    T: EventProcessor,
{
    if let Err(e) = consume_messages(vec![broker], topic, event_processor) {
        println!("Failed consuming messages: {}", e);
    }
}

pub trait EventProcessor {
    fn process(&self, value: serde_json::Value);
}

fn consume_messages<T>(
    brokers: Vec<String>,
    topic: String,
    event_processor: T,
) -> Result<(), KafkaError>
where
    T: EventProcessor,
{
    let mut con = Consumer::from_hosts(brokers)
        .with_topic(topic)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .expect("unable to connect");

    loop {
        let mss = con.poll().expect("unable to poll");
        if mss.is_empty() {
            println!("No messages available right now.");
            return Ok(());
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                println!("{}:{}@{}", ms.topic(), ms.partition(), m.offset);
                let value = std::str::from_utf8(m.value).expect("invalid UTF-8");
                let value: serde_json::Value = serde_json::from_str(&value).unwrap();
                event_processor.process(value);
            }
            let _ = con.consume_messageset(ms);
        }

        // NOTE: messages must be marked and commited as consumed to ensure only once delivery.
        con.commit_consumed().expect("unable to commit consumed");
    }
}
