#[derive(Debug)]
pub enum SurveyCaptureLayout {
    Generated, // Domains::SurveyDesign::SurveyCaptureLayout::Generated
    UnknownEvent,
}

impl SurveyCaptureLayout {
    pub fn build(event_type: &str, _body: serde_json::Value) -> Self {
        match event_type {
            "Generated" => SurveyCaptureLayout::Generated,
            _ => SurveyCaptureLayout::UnknownEvent,
        }
    }
}
