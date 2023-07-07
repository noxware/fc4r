use super::document::Document;

// TODO: Optimize this.
// TODO: Use an iterator instead of a vector.
pub fn query<'a>(documents: &'a Vec<Document>, prompt: &str) -> Vec<&'a Document> {
    documents.iter().filter(|d| check(d, prompt)).collect()
}

pub fn check(document: &Document, prompt: &str) -> bool {
    let searching_for: Vec<&str> = prompt.split_whitespace().collect();

    let mut matches = true;

    for label in searching_for.iter() {
        if !document.labels.iter().any(|l| l == label) {
            matches = false;
            break;
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::label::LabelSet;

    #[test]
    fn query_works() {
        let documents = vec![
            Document {
                name: "name1".into(),
                labels: LabelSet::from_iter(vec!["l1".into(), "l2".into()]),
            },
            Document {
                name: "name2".into(),
                labels: LabelSet::from_iter(vec!["l1".into()]),
            },
            Document {
                name: "name3".into(),
                labels: LabelSet::from_iter(vec!["l2".into()]),
            },
            Document {
                name: "name4".into(),
                labels: LabelSet::from_iter(vec!["l3".into()]),
            },
        ];

        let results = query(&documents, "l1 l2");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "name1");

        let results = query(&documents, "l1");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "name1");
        assert_eq!(results[1].name, "name2");

        let results = query(&documents, "l2");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "name1");
        assert_eq!(results[1].name, "name3");

        let results = query(&documents, "l3");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "name4");

        let results = query(&documents, "l1 l2 l3");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn check_works() {
        let document = Document {
            name: "name".into(),
            labels: LabelSet::from_iter(vec!["l1".into(), "l2".into()]),
        };

        assert!(check(&document, "l1"));
        assert!(check(&document, "l2"));
        assert!(check(&document, "l1 l2"));
        assert!(!check(&document, "l3"));
        assert!(!check(&document, "l1 l3"));
        assert!(!check(&document, "l2 l3"));
        assert!(!check(&document, "l1 l2 l3"));
    }
}
