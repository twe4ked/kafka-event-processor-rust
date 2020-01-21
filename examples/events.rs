mod survey;
mod survey_capture_layout;

use serde::Deserialize;
pub use survey::*;
pub use survey_capture_layout::*;

#[derive(Debug, Deserialize)]
pub struct LocalizedText {
    locale: String,
    text: String,
}

#[derive(Debug)]
pub enum DomainEvent {
    Survey(Survey),
    SurveyCaptureLayout(SurveyCaptureLayout),
    UnknownAggregate,
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
            let body = value.get("body").expect("unable to get body").to_owned();

            let (aggregate_type, event_type) = parse_event_type(&event_type);

            match aggregate_type {
                "Survey" => DomainEvent::Survey(match event_type {
                    "Created" => Survey::Created(
                        serde_json::from_value(body).expect("unable to parse SurveyCreatedBody"),
                    ),
                    _ => Survey::UnknownEvent,
                }),
                "SurveyCaptureLayout" => DomainEvent::SurveyCaptureLayout(match event_type {
                    "Generated" => SurveyCaptureLayout::Generated,
                    _ => SurveyCaptureLayout::UnknownEvent,
                }),
                _ => DomainEvent::UnknownAggregate,
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

/// Example:
///
///     Input: Domains::SurveyDesign::Survey::Created
///     Output: (Survey, Created)
fn parse_event_type(event_type: &str) -> (&str, &str) {
    let mut e = event_type.split("::");
    e.next().unwrap(); // Domains
    e.next().unwrap(); // SurveyDesign
    (
        // Survey
        e.next().unwrap(),
        // Created
        e.next().unwrap(),
    )
}
