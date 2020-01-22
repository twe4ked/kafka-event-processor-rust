use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug)]
pub enum Participant {
    Invited(InvitedBody),
    UnknownEvent,
}

#[derive(Debug, Deserialize)]
pub struct InvitedBody {
    survey_period_id: Uuid,
    employee_id: Uuid,
    invited_at: String,
}

impl Participant {
    pub fn build(event_type: &str, body: serde_json::Value) -> Self {
        dbg!(&body);
        match event_type {
            "Invited" => Participant::Invited(
                serde_json::from_value(body).expect("unable to parse SurveyCreatedBody"),
            ),
            _ => Participant::UnknownEvent,
        }
    }
}
