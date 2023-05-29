use std::collections::HashSet;

struct LabelDef {
    name: String,
    aliases: Vec<String>,
    implies: Vec<String>,
    description: String,
}

struct LabelLibrary {
    label_defs: Vec<LabelDef>,
}

impl LabelLibrary {
    fn build(defs: Vec<LabelDef>) -> Result<LabelLibrary, String> {
        Self::validate(&defs)?;
        Ok(LabelLibrary { label_defs: defs })
    }

    /// Validates that the label definitions are valid.
    ///
    /// This is a placeholder for now.
    fn validate(defs: &Vec<LabelDef>) -> Result<(), String> {
        Ok(())
    }

    fn get_label_def(&self, name: &str) -> Option<&LabelDef> {
        let def = self
            .label_defs
            .iter()
            .find(|l| l.name == name || l.aliases.contains(&name.to_string()))?;

        Some(def)
    }

    fn resolve(&self, name: &str) -> Option<&str> {
        let def = self.get_label_def(name)?;
        Some(&def.name)
    }

    fn expand_into<'a>(&'a self, labels: &mut HashSet<&'a str>, name: &str) -> Option<()> {
        let def = self.get_label_def(name)?;

        labels.insert(&def.name);

        for alias in def.aliases.iter() {
            labels.insert(alias);
        }

        for implied in def.implies.iter() {
            if !labels.contains(implied.as_str()) {
                self.expand_into(labels, implied)?;
            }
        }

        Some(())
    }

    fn expand(&self, name: &str) -> Option<Vec<&str>> {
        let mut labels = HashSet::new();
        self.expand_into(&mut labels, name)?;
        Some(labels.into_iter().collect())
    }

    fn expand_and_sort(&self, name: &str) -> Option<Vec<&str>> {
        let mut labels = self.expand(name)?;
        labels.sort();
        Some(labels)
    }

    fn expand_all(&self, names: &[&str]) -> Option<Vec<&str>> {
        let mut labels = HashSet::new();

        for name in names.iter() {
            self.expand_into(&mut labels, name)?;
        }

        let labels: Vec<&str> = labels.into_iter().collect();

        Some(labels)
    }

    fn expand_all_and_sort(&self, names: &[&str]) -> Option<Vec<&str>> {
        let mut labels = self.expand_all(names)?;
        labels.sort();
        Some(labels)
    }

    fn get_description(&self, name: &str) -> Option<&str> {
        let def = self.get_label_def(name)?;
        Some(&def.description)
    }

    fn get_aliases(&self, name: &str) -> Option<&Vec<String>> {
        let def = self.get_label_def(name)?;
        Some(&def.aliases)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_library() -> LabelLibrary {
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

        assert_eq!(library.resolve("cat").unwrap(), "cat");
        assert_eq!(library.resolve("kitty").unwrap(), "cat");
        assert_eq!(library.resolve("purrr").unwrap(), "cat");
        assert_eq!(library.resolve("adorable").unwrap(), "cute");
        assert_eq!(library.resolve("kawaii").unwrap(), "cute");
        assert_eq!(library.resolve("memes").unwrap(), "meme");

        assert!(library.resolve("crab").is_none());
    }

    #[test]
    fn expand_includes_aliases_and_implies_relationships() {
        let library = setup_library();

        let mut result = library.expand("tiger").unwrap();
        result.sort();

        let expected = vec![
            "adorable", "cat", "cute", "kawaii", "kitty", "pet", "purrr", "tiger",
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn expand_does_not_include_recursive_implies_relationships() {
        let library = setup_library();

        let mut result_1 = library.expand("rec_1").unwrap();
        result_1.sort();

        let mut result_2 = library.expand("rec_2").unwrap();
        result_2.sort();

        let expected = vec!["rec_1", "rec_2"];

        assert_eq!(result_1, expected);
        assert_eq!(result_2, expected);
    }

    #[test]
    fn expand_and_sort_works() {
        let library = setup_library();

        let result = library.expand_and_sort("tiger").unwrap();

        let expected = vec![
            "adorable", "cat", "cute", "kawaii", "kitty", "pet", "purrr", "tiger",
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn expand_all_works() {
        let library = setup_library();

        let mut result = library.expand_all(&["cat", "puppy"]).unwrap();
        result.sort();

        let expected = vec![
            "adorable", "cat", "cute", "dog", "kawaii", "kitty", "pet", "puppy", "purrr",
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn expand_all_and_sort_works() {
        let library = setup_library();
        let result = library.expand_all_and_sort(&["rec_1", "puppy"]).unwrap();
        let expected = vec![
            "adorable", "cute", "dog", "kawaii", "pet", "puppy", "rec_1", "rec_2",
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn get_description_works() {
        let library = setup_library();
        assert_eq!(library.get_description("puppy").unwrap(), "Dog related");
    }

    #[test]
    fn get_aliases_works() {
        let library = setup_library();
        let def = library.get_aliases("cat").unwrap();
        assert_eq!(def, &["kitty", "purrr"]);
    }
}
