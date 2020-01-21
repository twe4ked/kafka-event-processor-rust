use serde::Deserialize;
use uuid::Uuid;

use super::LocalizedText;

#[derive(Debug, Deserialize)]
pub struct SurveyCreatedBody {
    account_id: Uuid,
    created_at: String,
    name: Vec<LocalizedText>,
}

#[derive(Debug)]
pub enum Survey {
    Created(SurveyCreatedBody), // Domains::SurveyDesign::Survey::Created
    UnknownEvent,
}

impl Survey {
    pub fn build(event_type: &str, body: serde_json::Value) -> Self {
        match event_type {
            "Created" => Survey::Created(
                serde_json::from_value(body).expect("unable to parse SurveyCreatedBody"),
            ),
            _ => Survey::UnknownEvent,
        }
    }
}
