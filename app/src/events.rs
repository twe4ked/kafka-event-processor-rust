mod participant;
mod response;
mod survey;
mod survey_capture_layout;
mod survey_period;
mod survey_question;

pub use participant::*;
pub use response::*;
pub use survey::*;
pub use survey_capture_layout::*;
pub use survey_period::*;
pub use survey_question::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LocalizedText {
    locale: String,
    text: String,
}

#[derive(Debug)]
pub enum DomainEvent {
    Survey(Survey),
    SurveyCaptureLayout(SurveyCaptureLayout),
    SurveyQuestion(SurveyQuestion),
    SurveyPeriod(SurveyPeriod),
    Participant(Participant),
    Response(Response),
    UnknownAggregate,
}

#[derive(Debug)]
pub struct Event {
    pub aggregate_id: String,
    pub domain_event: DomainEvent,
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
                "Survey" => DomainEvent::Survey(Survey::build(event_type, body)),
                "SurveyQuestion" => {
                    DomainEvent::SurveyQuestion(SurveyQuestion::build(event_type, body))
                }
                "SurveyPeriod" => DomainEvent::SurveyPeriod(SurveyPeriod::build(event_type, body)),
                "SurveyCaptureLayout" => {
                    DomainEvent::SurveyCaptureLayout(SurveyCaptureLayout::build(event_type, body))
                }
                "Participant" => DomainEvent::Participant(Participant::build(event_type, body)),
                "Response" => DomainEvent::Response(Response::build(event_type, body)),
                _ => DomainEvent::UnknownAggregate,
            }
        };

        let aggregate_id = value
            .get("aggregate_id")
            .expect("unable to get aggregate_id")
            .as_str()
            .expect("not a string")
            .to_string();

        Event {
            aggregate_id,
            domain_event,
        }
    }
}

/// Example:
///
///     Input: Domains::SurveyDesign::Survey::Created
///     Output: (Survey, Created)
fn parse_event_type(event_type: &str) -> (&str, &str) {
    let mut e = event_type.split("::").skip(2); // Domains::SurveyDesign
    (e.next().unwrap(), e.next().unwrap()) // (Survey, Created)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_parsing() {
        let data = r#"
            {
                "event_type": "Domains::SurveyDesign::Survey::Created",
                "aggregate_id": "b918c780-1ec8-0138-3ca8-2cde48001122",
                "body": {
                    "account_id": "b918c780-1ec8-0138-3ca8-2cde48001122",
                    "created_at": "TODO",
                    "name": []
                }
            }"#;
        let value: serde_json::Value = serde_json::from_str(&data).unwrap();
        let event = Event::from(value);
    }
}
