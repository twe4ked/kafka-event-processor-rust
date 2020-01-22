#[derive(Debug)]
pub enum SurveyQuestion {
    AddedToSurvey,
    QuestionTypeChanged,
    SelectOptionAdded,
    SelectOptionRenamed,
    UnknownEvent,
}

impl SurveyQuestion {
    pub fn build(event_type: &str, _body: serde_json::Value) -> Self {
        match event_type {
            "AddedToSurvey" => SurveyQuestion::AddedToSurvey,
            "QuestionTypeChanged" => SurveyQuestion::QuestionTypeChanged,
            "SelectOptionAdded" => SurveyQuestion::SelectOptionAdded,
            "SelectOptionRenamed" => SurveyQuestion::SelectOptionRenamed,
            _ => SurveyQuestion::UnknownEvent,
        }
    }
}
