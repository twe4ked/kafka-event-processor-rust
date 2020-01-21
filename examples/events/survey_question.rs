#[derive(Debug)]
pub enum SurveyQuestion {
    AddedToSurvey,
    QuestionTypeChanged,
    SelectOptionAdded,
    SelectOptionRenamed,
    UnknownEvent,
}
