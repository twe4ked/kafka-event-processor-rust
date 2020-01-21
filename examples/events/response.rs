#[derive(Debug)]
pub enum Response {
    Started,
    RatingQuestionAnswered,
    Submitted,
    UnknownEvent,
}
