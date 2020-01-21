#[derive(Debug)]
pub enum SurveyPeriod {
    Launched,
    UnknownEvent,
}

impl SurveyPeriod {
    pub fn build(event_type: &str, _body: serde_json::Value) -> Self {
        match event_type {
            "Launched" => SurveyPeriod::Launched,
            _ => SurveyPeriod::UnknownEvent,
        }
    }
}
