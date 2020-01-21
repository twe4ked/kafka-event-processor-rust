mod events;

struct Processor;

impl event_processor::Processor<events::Event> for Processor {
    fn process(&self, event: events::Event) {
        match &event.domain_event {
            events::DomainEvent::Survey(domain_event) => match domain_event {
                events::Survey::Created(body) => {
                    dbg!(body);
                }
                events::Survey::UnknownEvent => {}
            },
            events::DomainEvent::SurveyCaptureLayout(domain_event) => match domain_event {
                events::SurveyCaptureLayout::Generated => {
                    dbg!(event);
                }
                events::SurveyCaptureLayout::UnknownEvent => {}
            },
            _ => {
                dbg!("not handled");
            }
        }
    }
}

fn main() {
    let broker = std::env::var("KAFKA_BROKER").expect("KAFKA_BROKER not provided");
    let topic = std::env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC not provided");
    let processor = Processor {};

    event_processor::run(broker, topic, processor);
}
