use super::label::LabelSet;
use std::path::Path;

const FILENAME_LABELS_DELIMITER: &str = " fn ";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Document {
    // Provisory name matching the `from_filename` function.
    pub path: String,
    pub labels: LabelSet,
    pub name: String,
}

impl Document {
    pub fn from_filename(path: &str) -> Self {
        let path = Path::new(path);
        let filename = match path.file_name() {
            Some(filename) => filename.to_string_lossy(),
            None => "".into(),
        };

        match filename.split_once(FILENAME_LABELS_DELIMITER) {
            Some((labels, name)) => {
                // Does not require trim.
                let labels = labels.split_whitespace().map(|s| s.to_string()).collect();
                Self {
                    labels,
                    // Shall I use lossy here?
                    path: path.to_str().unwrap().to_string(),
                    name: name.trim().to_string(),
                }
            }
            None => Self {
                labels: LabelSet::empty(),
                path: path.to_str().unwrap().to_string(),
                name: filename.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_filename_works() {
        let doc = Document::from_filename("path/to/   l1   l2  fn   name.ext  ");

        assert_eq!(doc.name, "name.ext");
        assert_eq!(doc.labels, LabelSet::from(["l1", "l2"]));
    }

    #[test]
    fn from_filename_works_without_labels() {
        let doc = Document::from_filename("path/to/name.ext");
        assert_eq!(doc.name, "name.ext");
        assert!(doc.labels.is_empty());
    }

    #[test]
    fn from_filename_works_with_empty_labels() {
        let doc = Document::from_filename("path/to/   fn   name.ext  ");
        assert_eq!(doc.name, "name.ext");
        assert!(doc.labels.is_empty());
    }

    #[test]
    fn from_filename_works_with_empty_name() {
        let doc = Document::from_filename("path/to/   l1   l2  fn   ");

        assert_eq!(doc.name, "");
        assert_eq!(doc.labels, LabelSet::from(["l1", "l2"]));
    }

    #[test]
    fn from_filename_works_with_empty_name_and_empty_labels() {
        let doc = Document::from_filename("path/to/   fn   ");
        assert_eq!(doc.name, "");
        assert!(doc.labels.is_empty());
    }
}
