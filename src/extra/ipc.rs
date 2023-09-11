use serde::{Deserialize, Serialize};
use serde_json::value::Value;

use crate::core::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct RawMessage {
    #[serde(rename = "type")]
    kind: String,
    payload: Value,
}

impl RawMessage {
    fn deserialize(input: &str) -> Self {
        if input.starts_with("{") {
            serde_json::from_str(input).unwrap()
        } else {
            RawMessage {
                kind: "path".to_string(),
                payload: serde_json::to_value(input).unwrap(),
            }
        }
    }

    fn serialize(&self) -> String {
        if self.kind == "path" {
            serde_json::from_value(self.payload.clone()).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

#[derive(Debug)]
pub enum Message {
    Config(Config),
    // TODO: Consider renaming this to "Line", "String", "TextLine", etc.
    Path(String),
}

impl Message {
    pub fn deserialize(input: &str) -> Self {
        let raw_message = RawMessage::deserialize(input);
        let payload = raw_message.payload;

        match raw_message.kind.as_str() {
            "config" => Message::Config(serde_json::from_value(payload).unwrap()),
            "path" => Message::Path(serde_json::from_value(payload).unwrap()),
            kind => panic!("Unknown message type '{}'", kind),
        }
    }

    pub fn serialize(&self) -> String {
        let raw_message = match self {
            Message::Config(config) => RawMessage {
                kind: "config".to_string(),
                payload: serde_json::to_value(config).unwrap(),
            },
            Message::Path(path) => RawMessage {
                kind: "path".to_string(),
                payload: serde_json::to_value(path).unwrap(),
            },
        };

        raw_message.serialize()
    }

    // TODO: Check if these are safe to use.
    pub fn send(writer: &mut impl std::io::Write, message: &Self) {
        let serialized = message.serialize();
        writeln!(writer, "{}", serialized).unwrap();
    }

    // TODO: Check if these are safe to use.
    pub fn recv(reader: &mut impl std::io::BufRead) -> Self {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        Self::deserialize(&input)
    }
}
