use super::label::{LabelLibrary, LabelSet};
use std::path::Path;

const FILENAME_LABELS_DELIMITER: &str = " fn ";

pub struct Document {
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
                    name: name.trim().to_string(),
                }
            }
            None => Self {
                labels: LabelSet::empty(),
                name: filename.to_string(),
            },
        }
    }

    pub fn expand(&mut self, library: &LabelLibrary) -> () {
        self.labels.expand_with(library);
    }
}

#[cfg(test)]
mod tests {
    // TODO: Place this shared util elsewhere?
    use super::super::label::tests::setup_library;
    use super::*;

    #[test]
    fn from_filename_works() {
        let doc = Document::from_filename("path/to/   l1   l2  fn   name.ext  ");
        let mut labels: Vec<String> = doc.labels.into_iter().collect();
        labels.sort();

        assert_eq!(doc.name, "name.ext");
        assert_eq!(labels, ["l1", "l2"]);
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
        let mut labels: Vec<String> = doc.labels.into_iter().collect();
        labels.sort();

        assert_eq!(doc.name, "");
        assert_eq!(labels, ["l1", "l2"]);
    }

    #[test]
    fn from_filename_works_with_empty_name_and_empty_labels() {
        let doc = Document::from_filename("path/to/   fn   ");
        assert_eq!(doc.name, "");
        assert!(doc.labels.is_empty());
    }

    #[test]
    fn expand_works() {
        let library = setup_library();
        let mut labels = LabelSet::from_iter(vec![
            "cat".into(),
            "kitty".into(),
            "puppy".into(),
            "rec_1".into(),
        ]);

        let mut doc = Document {
            name: "name.ext".into(),
            labels: labels.clone(),
        };

        doc.expand(&library);
        labels.expand_with(&library);

        let mut expected = labels.into_iter().collect::<Vec<String>>();
        let mut result = doc.labels.into_iter().collect::<Vec<String>>();
        expected.sort();
        result.sort();

        assert_eq!(result, expected);
    }
}
