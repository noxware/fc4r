use std::{error::Error, path::Path};

const FILENAME_LABELS_DELIMITER: &str = " fn ";

pub struct Document {
    pub labels: Vec<String>,
    pub name: String,
}

impl Document {
    // In the future, it may return an error, but currently does not.
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(path);
        let filename = match path.file_name() {
            Some(filename) => filename.to_string_lossy(),
            None => "".into(),
        };

        match filename.split_once(FILENAME_LABELS_DELIMITER) {
            Some((labels, name)) => {
                // Does not require trim.
                let labels = labels.split_whitespace().map(|s| s.to_string()).collect();
                Ok(Self {
                    labels,
                    name: name.trim().to_string(),
                })
            }
            None => Ok(Self {
                labels: Vec::new(),
                name: filename.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_file_works() {
        let doc = Document::from_file("path/to/   l1   l2  fn   name.ext  ").unwrap();
        assert_eq!(doc.name, "name.ext");
        assert_eq!(doc.labels, ["l1", "l2"]);
    }

    #[test]
    fn from_file_works_without_labels() {
        let doc = Document::from_file("path/to/name.ext").unwrap();
        assert_eq!(doc.name, "name.ext");
        assert!(doc.labels.is_empty());
    }

    #[test]
    fn from_file_works_with_empty_labels() {
        let doc = Document::from_file("path/to/   fn   name.ext  ").unwrap();
        assert_eq!(doc.name, "name.ext");
        assert!(doc.labels.is_empty());
    }

    #[test]
    fn from_file_works_with_empty_name() {
        let doc = Document::from_file("path/to/   l1   l2  fn   ").unwrap();
        assert_eq!(doc.name, "");
        assert_eq!(doc.labels, ["l1", "l2"]);
    }

    #[test]
    fn from_file_works_with_empty_name_and_empty_labels() {
        let doc = Document::from_file("path/to/   fn   ").unwrap();
        assert_eq!(doc.name, "");
        assert!(doc.labels.is_empty());
    }
}
