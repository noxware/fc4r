use super::label::LabelLibrary;
use std::path::Path;

const FILENAME_LABELS_DELIMITER: &str = " fn ";

pub struct Document {
    pub labels: Vec<String>,
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
                labels: Vec::new(),
                name: filename.to_string(),
            },
        }
    }

    pub fn expand(&mut self, library: &LabelLibrary) -> () {
        // TODO: Consider Cow or other stuff to avoid unnecessary allocations.
        self.labels = library
            .expand_all(self.labels.as_slice())
            .iter()
            .map(|s| s.to_string())
            .collect();
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
        assert_eq!(doc.name, "name.ext");
        assert_eq!(doc.labels, ["l1", "l2"]);
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
        assert_eq!(doc.labels, ["l1", "l2"]);
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
        let labels = vec!["cat".into(), "kitty".into(), "puppy".into(), "rec_1".into()];
        let mut doc = Document {
            name: "name.ext".into(),
            labels: labels.clone(),
        };

        doc.expand(&library);

        // For some reason sorting is not stable. Not important.
        let mut expected = library.expand_all(labels.as_slice());
        let mut result = doc.labels;
        expected.sort();
        result.sort();

        assert_eq!(result, expected);
    }
}
