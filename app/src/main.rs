mod events;

struct Processor;
use tracing::{event, span, Level};

impl framework::Processor<events::Event> for Processor {
    fn process(&self, event: events::Event) {
        let span = span!(Level::INFO, "process");
        let _enter = span.enter();

        match &event.domain_event {
            events::DomainEvent::Survey(domain_event) => match domain_event {
                events::Survey::Created(body) => event!(Level::INFO, "{:#?}", body),
                events::Survey::UnknownEvent => {}
            },
            events::DomainEvent::SurveyCaptureLayout(domain_event) => match domain_event {
                events::SurveyCaptureLayout::Generated => event!(Level::INFO, "{:#?}", event),
                events::SurveyCaptureLayout::UnknownEvent => {}
            },
            _ => event!(Level::DEBUG, "{:#?}", event),
        }
    }
}

fn main() {
    let broker = std::env::var("KAFKA_BROKER").expect("KAFKA_BROKER not provided");
    let topic = std::env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC not provided");
    let processor = Processor {};

    tracing_subscriber::fmt::init();

    framework::run(broker, topic, processor);
}
