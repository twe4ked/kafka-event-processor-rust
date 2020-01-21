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
}
