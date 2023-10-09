use serde::{Deserialize, Serialize};
use serde_json::value::Value;

use crate::core::{config::Config, document::Document};

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
                kind: "line".to_string(),
                payload: serde_json::to_value(input).unwrap(),
            }
        }
    }

    fn serialize(&self) -> String {
        if self.kind == "line" {
            serde_json::from_value(self.payload.clone()).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    Config(Config),
    // TODO: Consider renaming this to "Line", "String", "TextLine", etc.
    Line(String),
    Document(Document),
}

impl Message {
    pub fn deserialize(input: &str) -> Self {
        let raw_message = RawMessage::deserialize(input);
        let payload = raw_message.payload;

        match raw_message.kind.as_str() {
            "config" => Message::Config(serde_json::from_value(payload).unwrap()),
            "document" => Message::Document(serde_json::from_value(payload).unwrap()),
            "line" => Message::Line(serde_json::from_value(payload).unwrap()),
            kind => panic!("Unknown message type '{}'", kind),
        }
    }

    pub fn serialize(&self) -> String {
        let raw_message = match self {
            Message::Config(config) => RawMessage {
                kind: "config".to_string(),
                payload: serde_json::to_value(config).unwrap(),
            },
            Message::Document(document) => RawMessage {
                kind: "document".to_string(),
                payload: serde_json::to_value(document).unwrap(),
            },
            Message::Line(line) => RawMessage {
                kind: "line".to_string(),
                payload: serde_json::to_value(line).unwrap(),
            },
        };

        raw_message.serialize()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::label::{LabelLibrary, LabelSet};

    use super::*;

    #[test]
    fn serialize_line() {
        let msg = Message::Line("test".to_string());
        let serialized = msg.serialize();

        assert_eq!(serialized, "test");
    }

    #[test]
    fn deserialize_implicit_line() {
        let msg = Message::deserialize("test");
        let expected = Message::Line("test".to_string());

        assert_eq!(msg, expected);
    }

    #[test]
    fn deserialize_explicit_line() {
        let msg = Message::deserialize(r#"{"type": "line", "payload": "test"}"#);
        let expected = Message::Line("test".to_string());

        assert_eq!(msg, expected);
    }

    #[test]
    fn serialize_and_deserialize_document() {
        let document = Document {
            path: "path".to_string(),
            name: "name".to_string(),
            labels: LabelSet::from(["l1", "l2"]),
        };
        let msg = Message::Document(document.clone());
        let serialized = msg.serialize();
        let deserialized = Message::deserialize(&serialized);

        assert_eq!(msg, deserialized);
        assert_eq!(document.name, "name");
        assert_eq!(document.path, "path");
        assert_eq!(document.labels, LabelSet::from(["l1", "l2"]));
    }

    #[test]
    fn serialize_and_deserialize_config() {
        let library = LabelLibrary::from_toml(
            r#"[label]
            aliases = ["alias"]
            implies = ["implied"]
            description = "a label"

            [implied]"#,
        )
        .unwrap();

        let msg = Message::Config(Config {
            labels: library.clone(),
        });
        let serialized = msg.serialize();
        let deserialized = Message::deserialize(&serialized);

        assert_eq!(msg, deserialized);
        assert_eq!(library.resolve("alias"), "label");
        assert_eq!(library.resolve("implied"), "implied");
    }
}
