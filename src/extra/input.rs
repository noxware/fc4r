use crate::core::document::Document;
use std::io;

pub fn read_stdin_documents() -> impl Iterator<Item = Document> {
    io::stdin()
        .lines()
        .map(|l| l.expect("Can't read line from stdio"))
        .map(|l| Document::from_filename(&l))
}
