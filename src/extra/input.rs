use std::io;
use std::path::PathBuf;

use crate::core::document::Document;
use crate::utils::fs::get_unique_target;

pub fn read_documents(reader: impl io::BufRead) -> impl Iterator<Item = Document> {
    reader
        .lines()
        .map(|l| l.expect("Can't read line from input"))
        .map(|l| Document::from_filename(&l))
}

pub fn read_stdin_documents() -> impl Iterator<Item = Document> {
    read_documents(io::stdin().lock())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SourceTargetPair {
    pub source: PathBuf,
    pub target: PathBuf,
}

fn map_sources_to_target_folder(
    sources: impl Iterator<Item = PathBuf>,
    target_folder: PathBuf,
) -> impl Iterator<Item = SourceTargetPair> {
    sources.map(move |source| {
        let target = get_unique_target(&source, &target_folder);
        SourceTargetPair { source, target }
    })
}

fn map_input_sources_to_target_folder(
    reader: impl io::BufRead,
    target_folder: PathBuf,
) -> impl Iterator<Item = SourceTargetPair> {
    let sources = reader
        .lines()
        .map(|l| l.expect("Can't read line from input"))
        .map(|l| PathBuf::from(l));

    map_sources_to_target_folder(sources, target_folder)
}

pub fn map_stdin_sources_to_target_folder(
    target_folder: PathBuf,
) -> impl Iterator<Item = SourceTargetPair> {
    map_input_sources_to_target_folder(io::stdin().lock(), target_folder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::label::LabelSet;
    use std::path::Path;

    #[test]
    fn read_documents_works() {
        let input = "a b c fn file1.ext
        the path/to/la_la-la fn file2.ext";
        let documents: Vec<_> = read_documents(input.as_bytes()).collect();
        assert_eq!(
            documents,
            vec![
                Document {
                    path: "a b c fn file1.ext".to_string(),
                    name: "file1.ext".to_string(),
                    labels: LabelSet::from(["a", "b", "c"]),
                },
                Document {
                    // TODO: Should this be trimmed by Document?
                    path: "        the path/to/la_la-la fn file2.ext".to_string(),
                    name: "file2.ext".to_string(),
                    labels: LabelSet::from(["la_la-la"]),
                },
            ]
        )
    }

    #[test]
    fn map_input_sources_to_target_folder_works() {
        let input = "a b c fn file1.ext
        the path/to/la_la-la fn file2.ext";

        let target_folder = Path::new("target_folder");

        let pairs: Vec<_> =
            map_input_sources_to_target_folder(input.as_bytes(), target_folder.to_path_buf())
                .collect();

        let source_1 = Path::new("a b c fn file1.ext");
        let source_2 = Path::new("        the path/to/la_la-la fn file2.ext");

        assert_eq!(
            pairs,
            vec![
                SourceTargetPair {
                    source: source_1.to_path_buf(),
                    target: get_unique_target(source_1, target_folder),
                },
                SourceTargetPair {
                    source: source_2.to_path_buf(),
                    target: get_unique_target(source_2, target_folder),
                },
            ]
        )
    }
}
