#[derive(Debug)]
pub enum Response {
    Started,
    RatingQuestionAnswered,
    Submitted,
    UnknownEvent,
}

impl Response {
    pub fn build(event_type: &str, _body: serde_json::Value) -> Self {
        match event_type {
            "Started" => Response::Started,
            "RatingQuestionAnswered" => Response::RatingQuestionAnswered,
            "Submitted" => Response::Submitted,
            _ => Response::UnknownEvent,
        }
    }
}
