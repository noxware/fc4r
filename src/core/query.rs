use super::{
    document::Document,
    label::{LabelLibrary, LabelSet},
};

const PSEUDO_DELIMITER: &str = ":";

pub struct CheckParams<'a> {
    pub prompt: &'a str,
    pub document: &'a Document,
    pub library: &'a LabelLibrary,
}

pub fn check(params: &CheckParams) -> bool {
    let CheckParams {
        prompt,
        document,
        library,
    } = params;
    let searching_for: Vec<&str> = prompt.split_whitespace().collect();

    let mut matches = true;

    for label in searching_for.iter() {
        let mut extended_labels = document.labels.clone();
        extended_labels.expand_with(library);

        if !check_table(document, &extended_labels, label) {
            matches = false;
            break;
        }
    }

    matches
}

fn check_table(document: &Document, labels: &LabelSet, current_label: &str) -> bool {
    match current_label.split_once(PSEUDO_DELIMITER) {
        Some((prefix, suffix)) => check_pseudo(document, labels, prefix, suffix),
        None => check_presence(labels, current_label),
    }
}

fn check_pseudo(document: &Document, labels: &LabelSet, prefix: &str, suffix: &str) -> bool {
    // TODO: Refator each type of pseudo matcher into it's own matcher module.
    match (prefix, suffix) {
        ("system", "unlabeled") => labels.is_empty(),
        ("system", "labeled") => !labels.is_empty(),
        ("not", _) => !check_table(document, labels, suffix),
        ("explicit", _) => check_presence(&document.labels, suffix),
        _ => false,
    }
}

fn check_presence(labels: &LabelSet, current_label: &str) -> bool {
    labels.iter().any(|l| l == current_label)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::label::LabelSet;

    #[test]
    fn check_works() {
        let toml = r#"
            [label]
            aliases = ["alias"]
            implies = ["implied"]
            description = "a label"

            [implied]
        "#;

        let library = LabelLibrary::from_toml(toml).unwrap();

        let document = Document {
            path: "".into(),
            name: "name".into(),
            labels: LabelSet::from(["l1", "l2", "label"]),
        };

        let mut params = CheckParams {
            prompt: "",
            document: &document,
            library: &library,
        };

        params.prompt = "l1";
        assert!(check(&params));

        params.prompt = "l2";
        assert!(check(&params));

        params.prompt = "l1 l2";
        assert!(check(&params));

        params.prompt = "l3";
        assert!(!check(&params));

        params.prompt = "l1 l3";
        assert!(!check(&params));

        params.prompt = "l2 l3";
        assert!(!check(&params));

        params.prompt = "l1 l2 l3";
        assert!(!check(&params));

        params.prompt = "system:labeled";
        assert!(check(&params));

        params.prompt = "system:unlabeled";
        assert!(!check(&params));

        params.prompt = "not:system:labeled";
        assert!(!check(&params));

        params.prompt = "not:system:unlabeled";
        assert!(check(&params));

        params.prompt = "not:l1";
        assert!(!check(&params));

        params.prompt = "not:l3";
        assert!(check(&params));

        params.prompt = "explicit:l1";
        assert!(check(&params));

        params.prompt = "explicit:not:l3";
        assert!(!check(&params));

        params.prompt = "explicit:label";
        assert!(check(&params));

        params.prompt = "explicit:implied";
        assert!(!check(&params));
    }
}
