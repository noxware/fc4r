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

        if !check_table(&extended_labels, label) {
            matches = false;
            break;
        }
    }

    matches
}

// TODO: Test pseudo labels.
fn check_table(labels: &LabelSet, current_label: &str) -> bool {
    match current_label.split_once(PSEUDO_DELIMITER) {
        Some((prefix, suffix)) => check_pseudo(labels, prefix, suffix),
        None => check_presence(labels, current_label),
    }
}

fn check_pseudo(labels: &LabelSet, prefix: &str, suffix: &str) -> bool {
    // TODO: Refator each type of pseudo matcher into it's own matcher module.
    match (prefix, suffix) {
        ("system", "unlabeled") => labels.is_empty(),
        ("system", "labeled") => !labels.is_empty(),
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
        let document = Document {
            filename: "".into(),
            name: "name".into(),
            labels: LabelSet::from(["l1", "l2"]),
        };

        let mut params = CheckParams {
            prompt: "",
            document: &document,
            // TODO: Test some expansion.
            library: &LabelLibrary::empty(),
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
    }
}
