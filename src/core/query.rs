use super::document::Document;

// TODO: Optimize this.
// TODO: Use an iterator instead of a vector.
pub fn query<'a>(documents: &'a Vec<Document>, prompt: &str) -> Vec<&'a Document> {
    let searching_for: Vec<&str> = prompt.split_whitespace().collect();

    documents
        .iter()
        .filter(|d| {
            let mut found = true;

            for label in searching_for.iter() {
                if !d.labels.iter().any(|l| l == label) {
                    found = false;
                    break;
                }
            }

            found
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_works() {
        let documents = vec![
            Document {
                name: "name1".into(),
                labels: vec!["l1".into(), "l2".into()],
            },
            Document {
                name: "name2".into(),
                labels: vec!["l1".into()],
            },
            Document {
                name: "name3".into(),
                labels: vec!["l2".into()],
            },
            Document {
                name: "name4".into(),
                labels: vec!["l3".into()],
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
}
