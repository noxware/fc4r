use crate::core::document::Document;
use std::io;

pub fn read_documents(reader: impl io::BufRead) -> impl Iterator<Item = Document> {
    reader
        .lines()
        .map(|l| l.expect("Can't read line from input"))
        .map(|l| Document::from_filename(&l))
}

pub fn read_stdin_documents() -> impl Iterator<Item = Document> {
    read_documents(io::stdin().lock())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::label::LabelSet;

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
}
