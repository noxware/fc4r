use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::iter::FromIterator;
use std::iter::IntoIterator;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LabelSet(HashSet<String>);

impl LabelSet {
    pub fn empty() -> Self {
        Self(HashSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn inner_set(&self) -> &HashSet<String> {
        &self.0
    }

    pub fn expand_with(&mut self, library: &LabelLibrary) -> () {
        let input_labels = &self.0;
        let mut output_labels = HashSet::new();

        for l in input_labels {
            library.expand_into(&mut output_labels, &l);
        }

        self.0 = output_labels;
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<String> {
        self.0.iter()
    }
}

impl IntoIterator for LabelSet {
    type Item = String;
    type IntoIter = std::collections::hash_set::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// https://doc.rust-lang.org/std/iter/index.html#iterating-by-reference
impl<'a> IntoIterator for &'a LabelSet {
    type Item = &'a String;
    type IntoIter = std::collections::hash_set::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromIterator<String> for LabelSet {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let set: HashSet<String> = iter.into_iter().collect();
        LabelSet(set)
    }
}

impl<'a> FromIterator<&'a str> for LabelSet {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let set: HashSet<String> = iter.into_iter().map(|s| s.to_string()).collect();
        LabelSet(set)
    }
}

impl<const N: usize> From<[&str; N]> for LabelSet {
    fn from(array: [&str; N]) -> Self {
        Self::from_iter(array)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct LabelDef {
    name: String,
    aliases: Vec<String>,
    implies: Vec<String>,
    description: String,
}

#[derive(Deserialize)]
struct RawLabelDef {
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    implies: Vec<String>,
    #[serde(default)]
    description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct LabelLibrary {
    label_defs: Vec<LabelDef>,
}

impl LabelLibrary {
    fn build(defs: Vec<LabelDef>) -> Result<Self, Box<dyn Error>> {
        Self::validate(&defs)?;
        Ok(Self { label_defs: defs })
    }

    pub fn empty() -> Self {
        Self::build(Vec::new()).unwrap()
    }

    /// Validates that the label definitions are valid.
    ///
    /// This is a placeholder for now.
    fn validate(_defs: &Vec<LabelDef>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn label_names(&self) -> Vec<&str> {
        self.label_defs.iter().map(|l| l.name.as_str()).collect()
    }

    fn get_label_def(&self, name: &str) -> Option<&LabelDef> {
        let def = self
            .label_defs
            .iter()
            .find(|l| l.name == name || l.aliases.contains(&name.to_string()))?;

        Some(def)
    }

    pub fn resolve_known(&self, name: &str) -> Option<&str> {
        let def = self.get_label_def(name)?;
        Some(&def.name)
    }

    pub fn resolve<'a>(&'a self, name: &'a str) -> &str {
        match self.resolve_known(name) {
            Some(known_name) => known_name,
            None => name,
        }
    }

    pub fn is_known(&self, name: &str) -> bool {
        self.resolve_known(name).is_some()
    }

    // TODO: Consider moving into LabelSet using `get_label_def` from there.
    // Also consider adjusting the logic since some of this may no longer be necessary.
    fn expand_into(&self, labels: &mut HashSet<String>, name: &str) -> () {
        match self.get_label_def(name) {
            Some(def) => {
                labels.insert(def.name.clone());

                for alias in def.aliases.iter() {
                    labels.insert(alias.clone());
                }

                for implied in def.implies.iter() {
                    if !labels.contains(implied.as_str()) {
                        self.expand_into(labels, implied);
                    }
                }
            }
            None => {
                labels.insert(name.to_string());
            }
        }
    }

    pub fn get_description(&self, name: &str) -> &str {
        match self.get_label_def(name) {
            Some(def) => &def.description,
            None => "",
        }
    }

    pub fn get_aliases(&self, name: &str) -> &[String] {
        match self.get_label_def(name) {
            Some(def) => &def.aliases.as_slice(),
            None => &[],
        }
    }

    pub fn from_toml(toml: &str) -> Result<Self, Box<dyn Error>> {
        let raw_labels: HashMap<String, RawLabelDef> = toml::from_str(toml)?;
        let labels = raw_labels
            .into_iter()
            .map(|(name, raw)| LabelDef {
                name,
                aliases: raw.aliases,
                implies: raw.implies,
                description: raw.description,
            })
            .collect();

        Self::build(labels)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn setup_library() -> LabelLibrary {
        let labels = vec![
            LabelDef {
                name: "cute".to_string(),
                aliases: vec!["adorable".to_string(), "kawaii".to_string()],
                implies: vec![],
                description: "Something cute".to_string(),
            },
            LabelDef {
                name: "pet".to_string(),
                aliases: vec![],
                implies: vec!["cute".to_string()],
                description: "Pet related".to_string(),
            },
            LabelDef {
                name: "cat".to_string(),
                aliases: vec!["kitty".to_string(), "purrr".to_string()],
                implies: vec!["pet".to_string()],
                description: "Cat related".to_string(),
            },
            LabelDef {
                name: "tiger".to_string(),
                aliases: vec![],
                implies: vec!["cat".to_string()],
                description: "Tiger stuff".to_string(),
            },
            LabelDef {
                name: "dog".to_string(),
                aliases: vec!["puppy".to_string()],
                implies: vec!["pet".to_string()],
                description: "Dog related".to_string(),
            },
            LabelDef {
                name: "meme".to_string(),
                aliases: vec!["memes".to_string()],
                implies: vec![],
                description: "Something funny".to_string(),
            },
            LabelDef {
                name: "rec_1".to_string(),
                aliases: vec![],
                implies: vec!["rec_2".to_string()],
                description: "".to_string(),
            },
            LabelDef {
                name: "rec_2".to_string(),
                aliases: vec![],
                implies: vec!["rec_1".to_string()],
                description: "".to_string(),
            },
        ];

        LabelLibrary::build(labels).unwrap()
    }

    #[test]
    fn resolve_works_with_names_and_aliases() {
        let library = setup_library();

        assert_eq!(library.resolve("cat"), "cat");
        assert_eq!(library.resolve("kitty"), "cat");
        assert_eq!(library.resolve("purrr"), "cat");
        assert_eq!(library.resolve("adorable"), "cute");
        assert_eq!(library.resolve("kawaii"), "cute");
        assert_eq!(library.resolve("memes"), "meme");

        assert_eq!(library.resolve("unknown_label_name"), "unknown_label_name");
    }

    #[test]
    fn expand_with_includes_aliases_and_implies_relationships() {
        let library = setup_library();

        let mut result = LabelSet::from(["tiger"]);
        result.expand_with(&library);

        let expected = LabelSet::from([
            "adorable", "cat", "cute", "kawaii", "kitty", "pet", "purrr", "tiger",
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn expand_with_works_with_recursive_implies_relationships() {
        let library = setup_library();

        let mut result = LabelSet::from(["rec_1"]);
        result.expand_with(&library);

        let expected = LabelSet::from(["rec_1", "rec_2"]);

        assert_eq!(result, expected);
    }

    #[test]
    fn expand_with_works() {
        let library = setup_library();

        let mut result = LabelSet::from(["cat", "puppy"]);
        result.expand_with(&library);

        let expected = LabelSet::from([
            "adorable", "cat", "cute", "dog", "kawaii", "kitty", "pet", "puppy", "purrr",
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn expand_with_works_with_empty_input() {
        let library = setup_library();

        let mut result = LabelSet::from([]);
        result.expand_with(&library);

        let expected = LabelSet::from([]);

        assert_eq!(result, expected);
    }

    #[test]
    fn get_description_works() {
        let library = setup_library();
        assert_eq!(library.get_description("puppy"), "Dog related");
    }

    #[test]
    fn get_aliases_works() {
        let library = setup_library();
        let def = library.get_aliases("cat");
        assert_eq!(def, &["kitty", "purrr"]);
    }

    #[test]
    fn from_toml_works() {
        let toml = r#"
            [label]
            aliases = ["alias"]
            implies = ["implied"]
            description = "a label"

            [implied]
        "#;

        let labels = LabelLibrary::from_toml(toml).unwrap();

        let label_name = labels.resolve("alias");
        let label_description = labels.get_description("label");

        assert_eq!(label_name, "label");
        assert_eq!(label_description, "a label");
    }

    #[test]
    fn resolve_known_works() {
        let library = setup_library();

        assert_eq!(library.resolve_known("cat"), Some("cat"));
        assert_eq!(library.resolve_known("purrr"), Some("cat"));

        assert_eq!(library.resolve_known("unknown_label_name"), None);
    }

    #[test]
    fn label_names_works() {
        let library = setup_library();

        let mut result = library.label_names();
        result.sort();

        let expected = vec![
            "cat", "cute", "dog", "meme", "pet", "rec_1", "rec_2", "tiger",
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn is_known_works() {
        let library = setup_library();

        assert_eq!(library.is_known("cat"), true);
        assert_eq!(library.is_known("purrr"), true);

        assert_eq!(library.is_known("unknown_label_name"), false);
    }
}
