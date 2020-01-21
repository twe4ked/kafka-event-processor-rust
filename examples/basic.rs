mod events {
    #[derive(Debug)]
    pub enum Survey {
        Created, // Domains::SurveyDesign::Survey::Created
    }

    #[derive(Debug)]
    pub enum SurveyCaptureLayout {
        Generated, // Domains::SurveyDesign::SurveyCaptureLayout::Generated
    }

    #[derive(Debug)]
    pub enum DomainEvent {
        Survey(Survey),
        SurveyCaptureLayout(SurveyCaptureLayout),
        Unknown,
    }

    impl From<&str> for DomainEvent {
        fn from(value: &str) -> Self {
            match value {
                "Domains::SurveyDesign::Survey::Created" => DomainEvent::Survey(Survey::Created),
                "Domains::SurveyDesign::SurveyCaptureLayout::Generated" => {
                    DomainEvent::SurveyCaptureLayout(SurveyCaptureLayout::Generated)
                }
                _ => DomainEvent::Unknown,
            }
        }
    }

    #[derive(Debug)]
    pub struct Event {
        pub aggregate_id: String,
        pub domain_event: DomainEvent,
        pub body: serde_json::Value,
    }

    impl From<serde_json::Value> for Event {
        fn from(value: serde_json::Value) -> Self {
            let domain_event = {
                let event_type = value
                    .get("event_type")
                    .expect("unable to get event_type")
                    .as_str()
                    .expect("not a string");
                DomainEvent::from(event_type)
            };

            Event {
                aggregate_id: value
                    .get("aggregate_id")
                    .expect("unable to get aggregate_id")
                    .as_str()
                    .expect("not a string")
                    .to_string(),
                domain_event: domain_event,
                body: value.get("body").expect("unable to get body").to_owned(),
            }
        }
    }
}

struct Processor;

impl event_processor::Processor<events::Event> for Processor {
    fn process(&self, event: events::Event) {
        match &event.domain_event {
            events::DomainEvent::Survey(domain_event) => match domain_event {
                events::Survey::Created => {
                    dbg!(event);
                }
            },
            events::DomainEvent::SurveyCaptureLayout(domain_event) => match domain_event {
                events::SurveyCaptureLayout::Generated => {
                    dbg!(event);
                }
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
