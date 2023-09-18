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

        if !check_table(document, &extended_labels, label, &library) {
            matches = false;
            break;
        }
    }

    matches
}

fn check_table(
    document: &Document,
    labels: &LabelSet,
    current_label: &str,
    library: &LabelLibrary,
) -> bool {
    match current_label.split_once(PSEUDO_DELIMITER) {
        Some((prefix, suffix)) => check_pseudo(document, labels, prefix, suffix, library),
        None => check_presence(labels, current_label),
    }
}

fn check_pseudo(
    document: &Document,
    labels: &LabelSet,
    prefix: &str,
    suffix: &str,
    library: &LabelLibrary,
) -> bool {
    // TODO: Refator each type of pseudo matcher into it's own matcher module.
    match (prefix, suffix) {
        // TODO: If this is the negation of `labeled` consider removing it.
        ("system", "unlabeled") => labels.is_empty(),
        ("system", "labeled") => !labels.is_empty(),
        // TODO: If this is the negation of `known` consider removing it.
        ("system", "unknown") => labels.iter().any(|l| !library.is_known(l)),
        ("system", "known") => labels.iter().any(|l| library.is_known(l)),
        ("not", _) => !check_table(document, labels, suffix, library),
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

    fn make_document() -> Document {
        Document {
            path: "".into(),
            name: "".into(),
            labels: LabelSet::from(["l1", "l2", "label"]),
        }
    }

    fn make_library() -> LabelLibrary {
        let toml = format!(
            r#"
            [label]
            aliases = ["alias"]
            implies = ["implied"]
            description = "a label"

            [implied]
        "#,
        );

        LabelLibrary::from_toml(&toml).unwrap()
    }

    fn assert_check(prompt: &str, expected: bool, document: &Document, library: &LabelLibrary) {
        let params = CheckParams {
            prompt,
            document,
            library,
        };

        assert_eq!(check(&params), expected);
    }

    #[test]
    fn check_with_unknown_labels() {
        let library = make_library();
        let document = make_document();
        let ac = |prompt: &str, expected: bool| assert_check(prompt, expected, &document, &library);

        ac("l1", true);
        ac("l2", true);
        ac("l1 l2", true);
        ac("l3", false);
        ac("l1 l3", false);
        ac("l2 l3", false);
        ac("l1 l2 l3", false);
    }

    #[test]
    fn check_with_system_labeled() {
        let library = make_library();
        let document = make_document();
        let ac = |prompt: &str, expected: bool| assert_check(prompt, expected, &document, &library);

        ac("system:labeled", true);
        ac("system:unlabeled", false);
        ac("not:system:labeled", false);
        ac("not:system:unlabeled", true);
    }

    #[test]
    fn check_with_unknown_not() {
        let library = make_library();
        let document = make_document();
        let ac = |prompt: &str, expected: bool| assert_check(prompt, expected, &document, &library);

        ac("not:l1", false);
        ac("not:l3", true);
    }

    #[test]
    fn check_with_explicit() {
        let library = make_library();
        let document = make_document();
        let ac = |prompt: &str, expected: bool| assert_check(prompt, expected, &document, &library);

        ac("explicit:l1", true);
        ac("explicit:not:l3", false);
        ac("explicit:label", true);
        ac("explicit:implied", false);
    }
}
