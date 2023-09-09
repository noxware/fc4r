use crate::utils::fs::get_prefix;

use super::label::LabelSet;
use std::path::Path;

const DEFAULT_LABELS_DELIMITER: &str = " fn ";
const RIGHT_LABELS_DELIMITER: &str = " fr ";
const RIGHT_LABELS_TOGGLE: &str = "fr ";

enum LabelPlacement {
    Left,
    /// Labels on the right, BUT still using `fn` as delimiter.
    RightByFn,
    /// Labels on the right using `fr` as delimiter.
    RightByFr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Document {
    // Provisory name matching the `from_filename` function.
    pub path: String,
    pub labels: LabelSet,
    pub name: String,
}

/// Util to remove the `fr` prefix mostly when present and trim.
fn clean_name(s: &str) -> &str {
    if s.starts_with(RIGHT_LABELS_TOGGLE) {
        &s[RIGHT_LABELS_TOGGLE.len()..].trim()
    } else {
        s.trim()
    }
}

impl Document {
    pub fn from_filename(path: &str) -> Self {
        let path = Path::new(path);
        let filename_prefix = get_prefix(path);

        let labels_placement = if filename_prefix.contains(RIGHT_LABELS_TOGGLE) {
            if filename_prefix.starts_with(RIGHT_LABELS_TOGGLE) {
                LabelPlacement::RightByFn
            } else {
                LabelPlacement::RightByFr
            }
        } else {
            LabelPlacement::Left
        };

        let delimiter = match labels_placement {
            LabelPlacement::Left => DEFAULT_LABELS_DELIMITER,
            LabelPlacement::RightByFn => DEFAULT_LABELS_DELIMITER,
            LabelPlacement::RightByFr => RIGHT_LABELS_DELIMITER,
        };

        match filename_prefix.split_once(delimiter) {
            Some(parts) => {
                let labels = match labels_placement {
                    LabelPlacement::Left => parts.0,
                    _ => parts.1,
                };

                let name = match labels_placement {
                    LabelPlacement::Left => parts.1,
                    _ => parts.0,
                };

                // Does not require trim.
                let labels = labels.split_whitespace().map(|s| s.to_string()).collect();
                Self {
                    labels,
                    // Shall I use lossy here?
                    path: path.to_str().unwrap().to_string(),
                    name: clean_name(name).to_string(),
                }
            }
            None => Self {
                labels: LabelSet::empty(),
                path: path.to_str().unwrap().to_string(),
                name: filename_prefix.to_string(),
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

        assert_eq!(doc.name, "name");
        assert_eq!(doc.labels, LabelSet::from(["l1", "l2"]));
    }

    #[test]
    fn from_filename_works_without_labels() {
        let doc = Document::from_filename("path/to/name.ext");
        assert_eq!(doc.name, "name");
        assert!(doc.labels.is_empty());
    }

    #[test]
    fn from_filename_works_with_empty_labels() {
        let doc = Document::from_filename("path/to/   fn   name.ext  ");
        assert_eq!(doc.name, "name");
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

    #[test]
    fn from_filename_works_with_right_fr_labels() {
        let doc = Document::from_filename("path/to/  name    fr   l1   l2   .ext   ");

        assert_eq!(doc.name, "name");
        assert_eq!(doc.labels, LabelSet::from(["l1", "l2"]));
    }

    #[test]
    fn from_filename_works_with_right_fr_labels_and_empty_name() {
        // Warning: If `fr` is at the beginning, it will be considered a "right fn" case.
        let doc = Document::from_filename("path/to/  fr   l1   l2   .ext   ");

        assert_eq!(doc.name, "");
        assert_eq!(doc.labels, LabelSet::from(["l1", "l2"]));
    }

    #[test]
    fn from_filename_works_with_right_fr_labels_and_empty_labels() {
        let doc = Document::from_filename("path/to/  name    fr   .ext   ");

        assert_eq!(doc.name, "name");
        assert!(doc.labels.is_empty());
    }

    #[test]
    fn from_filename_works_with_right_fr_labels_and_empty_name_and_empty_labels() {
        let doc = Document::from_filename("path/to/  fr   .ext   ");

        assert_eq!(doc.name, "");
        assert!(doc.labels.is_empty());
    }

    #[test]
    fn from_filename_works_with_right_fn_labels() {
        // Should handle empty space before `fr`?
        let doc = Document::from_filename("path/to/fr  name   fn    l1   l2   .ext   ");

        assert_eq!(doc.name, "name");
        assert_eq!(doc.labels, LabelSet::from(["l1", "l2"]));
    }

    #[test]
    fn from_filename_works_with_right_fn_labels_and_empty_name() {
        let doc = Document::from_filename("path/to/fr   fn    l1   l2   .ext   ");

        assert_eq!(doc.name, "");
        assert_eq!(doc.labels, LabelSet::from(["l1", "l2"]));
    }

    #[test]
    fn from_filename_works_with_right_fn_labels_and_empty_labels() {
        let doc = Document::from_filename("path/to/fr  name   fn    .ext   ");

        assert_eq!(doc.name, "name");
        assert!(doc.labels.is_empty());
    }

    #[test]
    fn from_filename_works_with_right_fn_labels_and_empty_name_and_empty_labels() {
        let doc = Document::from_filename("path/to/fr   fn    .ext   ");

        assert_eq!(doc.name, "");
        assert!(doc.labels.is_empty());
    }
}
