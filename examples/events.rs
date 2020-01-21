mod participant;
mod response;
mod survey;
mod survey_capture_layout;
mod survey_period;
mod survey_question;

pub use participant::*;
pub use response::*;
use serde::Deserialize;
pub use survey::*;
pub use survey_capture_layout::*;
pub use survey_period::*;
pub use survey_question::*;

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
                "Survey" => DomainEvent::Survey(match event_type {
                    "Created" => Survey::Created(
                        serde_json::from_value(body).expect("unable to parse SurveyCreatedBody"),
                    ),
                    _ => panic!("{}:{}", aggregate_type, event_type),
                }),
                "SurveyQuestion" => DomainEvent::SurveyQuestion(match event_type {
                    "AddedToSurvey" => SurveyQuestion::AddedToSurvey,
                    "QuestionTypeChanged" => SurveyQuestion::QuestionTypeChanged,
                    "SelectOptionAdded" => SurveyQuestion::QuestionTypeChanged,
                    "SelectOptionRenamed" => SurveyQuestion::SelectOptionRenamed,
                    _ => panic!("{}:{}", aggregate_type, event_type),
                }),
                "SurveyPeriod" => DomainEvent::SurveyPeriod(match event_type {
                    "Launched" => SurveyPeriod::Launched,
                    _ => panic!("{}:{}", aggregate_type, event_type),
                }),
                "SurveyCaptureLayout" => DomainEvent::SurveyCaptureLayout(match event_type {
                    "Generated" => SurveyCaptureLayout::Generated,
                    _ => SurveyCaptureLayout::UnknownEvent,
                }),
                "Participant" => DomainEvent::Participant(match event_type {
                    "Invited" => Participant::Invited,
                    _ => panic!("{}:{}", aggregate_type, event_type),
                }),
                "Response" => DomainEvent::Response(match event_type {
                    "Started" => Response::Started,
                    "RatingQuestionAnswered" => Response::RatingQuestionAnswered,
                    "Submitted" => Response::Submitted,
                    _ => panic!("{}:{}", aggregate_type, event_type),
                }),
                _ => panic!("{}:{}", aggregate_type, event_type),
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
