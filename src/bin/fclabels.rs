use tabled::{
    settings::{object::Rows, Modify, Style, Width},
    Table, Tabled,
};

use clap::{Parser, ValueEnum};

use fileclass::{core::label::LabelDef, extra::ipc::Message};
use fileclass::{core::label::LabelLibrary, extra::input::read_stdin_messages};

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    Table,
    Toml,
}

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// Use a specific output format
    #[arg(value_enum, short, long, default_value_t=Format::Table)]
    format: Format,
}

#[derive(Tabled)]
struct Row {
    name: String,
    aliases: String,
    description: String,
}

fn main() {
    let args = Args::parse();

    let mut library = LabelLibrary::empty();
    let mut known_library = LabelLibrary::empty();
    let mut unknown_library = LabelLibrary::empty();

    for msg in read_stdin_messages() {
        match msg {
            Message::Config(c) => {
                library = c.labels;
            }
            Message::Document(d) => {
                for label in d.labels {
                    if library.is_known(&label) {
                        if !known_library.is_known(&label) {
                            known_library.define(library.get_label_def(&label).unwrap().clone());
                        }
                    } else {
                        if !unknown_library.is_known(&label) {
                            unknown_library.define(LabelDef {
                                name: label,
                                implies: Vec::new(),
                                aliases: Vec::new(),
                                description: "Unknown label".to_string(),
                            });
                        }
                    }
                }
            }
            _ => panic!("Unexpected message"),
        }
    }

    match args.format {
        Format::Table => print_table(known_library, unknown_library),
        Format::Toml => print_toml(known_library, unknown_library),
    }
}

fn print_toml(known_library: LabelLibrary, unknown_library: LabelLibrary) {
    let known_toml = known_library.to_toml();
    let unknown_toml = unknown_library.to_toml();

    println!("{}", known_toml);
    println!("{}", unknown_toml);
}

fn print_table(known_library: LabelLibrary, unknown_library: LabelLibrary) {
    let mut known_rows: Vec<Row> = Vec::new();
    let mut unknown_rows: Vec<Row> = Vec::new();

    for name in known_library.label_names() {
        let mut aliases = Vec::from(known_library.get_aliases(name));
        aliases.sort();

        let aliases = aliases.join("\n");

        let row = Row {
            name: name.to_string(),
            aliases,
            description: known_library.get_description(name).to_string(),
        };

        known_rows.push(row);
    }

    for name in unknown_library.label_names() {
        let row = Row {
            name: name.to_string(),
            aliases: "".to_string(),
            description: "Unknown label".to_string(),
        };

        unknown_rows.push(row);
    }

    known_rows.sort_by(|a, b| a.name.cmp(&b.name));
    unknown_rows.sort_by(|a, b| a.name.cmp(&b.name));

    let rows = known_rows.into_iter().chain(unknown_rows.into_iter());

    let mut table = Table::new(rows);

    // TODO: Use percents per col instead of fixed values.

    table.with(Style::modern());
    table.with(Modify::new(Rows::first()).with(Width::wrap(25).keep_words()));
    table.with(Modify::new(Rows::new(1..=1)).with(Width::wrap(25).keep_words()));
    table.with(Modify::new(Rows::new(2..)).with(Width::wrap(100).keep_words()));

    // table.with(Width::wrap(Percent(90)));

    println!("{}", table.to_string());
}
