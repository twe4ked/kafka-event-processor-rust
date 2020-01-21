#[derive(Debug)]
pub enum Participant {
    Invited,
    UnknownEvent,
}

impl Participant {
    pub fn build(event_type: &str, _body: serde_json::Value) -> Self {
        match event_type {
            "Invited" => Participant::Invited,
            _ => Participant::UnknownEvent,
        }
    }
}
