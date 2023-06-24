use std::io;

use tabled::{
    settings::{object::Rows, Modify, Style, Width},
    Table, Tabled,
};

use fileclass::core::{config::Config, document::Document};

#[derive(Tabled)]
struct Row {
    name: String,
    aliases: String,
    description: String,
}

fn main() {
    let config = Config::std_load().unwrap();
    let library = config.labels;

    let mut rows: Vec<Row> = Vec::new();
    let mut names = library.label_names();
    names.sort();

    for name in names.iter() {
        let mut aliases = Vec::from(library.get_aliases(name));
        aliases.sort();

        let aliases = aliases.join("\n");

        rows.push(Row {
            name: name.to_string(),
            aliases,
            description: library.get_description(name).to_string(),
        });
    }

    let mut unknown_labels: Vec<String> = io::stdin()
        .lines()
        .map(|line| line.expect("Can't read line from stdio"))
        .flat_map(|line| Document::from_filename(&line).labels)
        .filter(|label| !library.is_known(label))
        .collect();

    unknown_labels.sort();
    unknown_labels.dedup(); // Works thanks to the sort.

    for label in unknown_labels.iter() {
        rows.push(Row {
            name: label.to_string(),
            aliases: "".to_string(),
            description: "Unknown label".to_string(),
        });
    }

    let mut table = Table::new(rows);

    // TODO: Use percents per col instead of fixed values.

    table.with(Style::modern());
    table.with(Modify::new(Rows::first()).with(Width::wrap(25).keep_words()));
    table.with(Modify::new(Rows::new(1..=1)).with(Width::wrap(25).keep_words()));
    table.with(Modify::new(Rows::new(2..)).with(Width::wrap(100).keep_words()));

    // table.with(Width::wrap(Percent(90)));

    println!("{}", table.to_string());
}
