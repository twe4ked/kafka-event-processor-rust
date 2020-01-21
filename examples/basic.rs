mod events {
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Debug, Deserialize)]
    pub struct SurveyCreatedBody {
        account_id: Uuid,
        created_at: String,
        name: Vec<LocalizedText>,
    }

    #[derive(Debug, Deserialize)]
    pub struct LocalizedText {
        locale: String,
        text: String,
    }

    #[derive(Debug)]
    pub enum Survey {
        Created(SurveyCreatedBody), // Domains::SurveyDesign::Survey::Created
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

                match event_type {
                    "Domains::SurveyDesign::Survey::Created" => {
                        let body: SurveyCreatedBody = serde_json::from_value(
                            value.get("body").expect("unable to get body").to_owned(),
                        )
                        .expect("unable to parse SurveyCreatedBody");
                        DomainEvent::Survey(Survey::Created(body))
                    }
                    "Domains::SurveyDesign::SurveyCaptureLayout::Generated" => {
                        DomainEvent::SurveyCaptureLayout(SurveyCaptureLayout::Generated)
                    }
                    _ => DomainEvent::Unknown,
                }
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
                events::Survey::Created(body) => {
                    dbg!(body);
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
