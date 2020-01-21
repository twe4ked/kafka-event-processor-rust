use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;

pub fn run<T, E>(broker: String, topic: String, processor: T)
where
    T: Processor<E>,
    E: From<serde_json::value::Value>,
{
    if let Err(e) = consume_messages(vec![broker], topic, processor) {
        println!("Failed consuming messages: {}", e);
    }
}

pub trait Processor<E> {
    fn process(&self, event: E);
}

fn consume_messages<T, E>(
    brokers: Vec<String>,
    topic: String,
    processor: T,
) -> Result<(), KafkaError>
where
    T: Processor<E>,
    E: From<serde_json::value::Value>,
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
                let event = E::from(value);
                processor.process(event);
            }
            let _ = con.consume_messageset(ms);
        }

        // NOTE: messages must be marked and commited as consumed to ensure only once delivery.
        con.commit_consumed().expect("unable to commit consumed");
    }
}
