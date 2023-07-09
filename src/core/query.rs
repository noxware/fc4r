use super::{document::Document, label::LabelLibrary};

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

        if !extended_labels.iter().any(|l| l == label) {
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
