use std::{env, io::BufRead, process::ExitCode};

const USAGE: &str = r#"
Usage: fretsmart [COMMANDS | SELECTION_OPTIONS] [DATA_FILE]

DATA_FILE                Use this instead of the default data file,
                         (~/connfig/fretsmart/data.json).

COMMANDS:
  list [OPTION]          Lists entries from DATA_FILE.
      instruments
      tunings
      highlights
      datafile           List entire DATA_FILE.
  update                 Add/modify/delete entries from DATA_FILE.

SELECTION_OPTIONS:       Default values are taken from DATA_FILE.
  --instrument <name>
  --tuning <name>
  --tuning-note <name>
  --highlight-type <name>
  --highlight <name>
  --highlight-note <name>

-h, --help               Display this information.
-v, --version            Print version information and quit.
"#;

const SELECTION_OPTION_VALUES: [&'static str; 6] = [
    "--instrument",
    "--tuning",
    "--tuning-note",
    "--highlight-type",
    "--highlight",
    "--highlight-note",
];
const SELECTION_OPTION_DEFAULTS: [&'static str; 6] = ["guitar", "std", "E", "S", "major", "C"];

#[derive(Debug)]
struct Selection {
    instrument: String,
    tuning: String,
    tuning_note: String,
    highlight_type: String,
    highlight: String,
    highlight_note: String,
}

impl Selection {
    fn new() -> Selection {
        Selection {
            instrument: SELECTION_OPTION_DEFAULTS.get(0).unwrap().to_string(),
            tuning: SELECTION_OPTION_DEFAULTS.get(1).unwrap().to_string(),
            tuning_note: SELECTION_OPTION_DEFAULTS.get(2).unwrap().to_string(),
            highlight_type: SELECTION_OPTION_DEFAULTS.get(3).unwrap().to_string(),
            highlight: SELECTION_OPTION_DEFAULTS.get(4).unwrap().to_string(),
            highlight_note: SELECTION_OPTION_DEFAULTS.get(5).unwrap().to_string(),
        }
    }
}

const DEFAULT_DATA_FILE: &'static str = "data.txt"; //    "~/.config/fretsmart/data.txt";

fn get_data_file_name(data_file: Option<&String>) -> &str {
    match data_file {
        Some(df) => df.as_str(),
        None => DEFAULT_DATA_FILE,
    }
}

#[derive(Debug)]
enum DataLine {
    Notes(Vec<String>),
    Instrument(String),
    Tuning {
        instrument: String,
        name: String,
        formula: Vec<String>,
    },
    Highlight {
        h_type: String,
        name: String,
        formula: Vec<String>,
    },
}

fn verify_data_line(data_line: &String) -> Result<DataLine, String> {
    let fields: Vec<&str> = data_line.split(",").collect();
    match fields.len() {
        2 => match fields.get(0).unwrap() {
            &"N" => {
                let notes: Vec<String> = fields
                    .get(1)
                    .unwrap()
                    .split(";")
                    .map(|v| v.to_string())
                    .collect();
                Ok(DataLine::Notes(notes))
            }
            &"I" => Ok(DataLine::Instrument(fields.get(1).unwrap().to_string())),
            _ => Err("invalid row".to_string()),
        },
        4 => match fields.get(0).unwrap() {
            &"T" => Ok(DataLine::Tuning {
                instrument: fields.get(1).unwrap().to_string(),
                name: fields.get(2).unwrap().to_string(),
                formula: fields
                    .get(3)
                    .unwrap()
                    .split(";")
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>(),
            }),
            &"H" => Ok(DataLine::Highlight {
                h_type: fields.get(1).unwrap().to_string(),
                name: fields.get(2).unwrap().to_string(),
                formula: fields
                    .get(3)
                    .unwrap()
                    .split(";")
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>(),
            }),
            _ => Err("invalid row".to_string()),
        },
        _ => Err("invalid row".to_string()),
    }
}

fn process_command_list(selected_list_option: &str, data_file: &str) {
    let prefix = match selected_list_option {
        "instruments" => "I,",
        "tunings" => "T,",
        "highlights" => "H,",
        _ => "D", // only value is "datafile".
    };

    let data_file = std::fs::File::open(data_file).unwrap();
    let data_file_lines = std::io::BufReader::new(data_file).lines().flatten();
    for line in data_file_lines {
        if line.starts_with(prefix) || prefix.eq("D") {
            match verify_data_line(&line) {
                Ok(fields) => match fields {
                    DataLine::Notes(notes) => println!("{:?}", notes),
                    DataLine::Instrument(instrument) => println!("{}", instrument),
                    DataLine::Tuning {
                        instrument,
                        name,
                        formula,
                    } => println!("{instrument} {name} {formula:?}"),
                    DataLine::Highlight {
                        h_type,
                        name,
                        formula,
                    } => println!("{h_type} {name} {formula:?}"),
                },
                Err(err) => {
                    eprintln!("{err} - {line}");
                    break;
                }
            }
        }
    }
}

fn process_command_update(data_file: &str) {
    println!("Updating {data_file}");
}

fn print_fretboard(selection: Selection, data: [Option<DataLine>; 4]) {
    if let Some(DataLine::Notes(notes)) = data.get(0).unwrap() {
        let tuning_note_idx = notes
            .iter()
            .position(|n| n.eq(&selection.tuning_note))
            .unwrap();
        let highlight_note_idx = notes
            .iter()
            .position(|n| n.eq(&selection.highlight_note))
            .unwrap();

        let mut highlight_notes = Vec::<String>::new();
        if let Some(DataLine::Highlight {
            h_type: _,
            name: _,
            formula: h_notes,
        }) = data.get(3).unwrap()
        {
            for h_formula in h_notes.iter() {
                let note = notes
                    .get((h_formula.parse::<usize>().unwrap_or(0) + highlight_note_idx) % 12)
                    .unwrap();
                highlight_notes.push(note.to_owned());
            }
        }

        if let Some(DataLine::Tuning {
            instrument: _,
            name: _,
            formula: t_notes,
        }) = data.get(2).unwrap()
        {
            for i in 0..=24 {
                print!("{i:^3}|");

                if vec![0, 3, 5, 7, 12, 15, 17, 19, 24].contains(&i) {
                    print!("|");
                }
            }
            println!();

            for t_formula in t_notes.iter() {
                for i in 0..=24 {
                    let note = notes
                        .get((t_formula.parse::<usize>().unwrap_or(0) + tuning_note_idx + i) % 12)
                        .unwrap();
                    if highlight_notes.contains(note) {
                        print!(" \x1b[1;31m{}\x1b[0m |", note);
                    } else {
                        print!(" {} |", note);
                    }
                    if vec![0, 3, 5, 7, 12, 15, 17, 19, 24].contains(&i) {
                        print!("|");
                    }
                }
                println!();
            }
        }
    }
}

fn process_selection_options(selected_options: Vec<(&str, &str)>, data_file_name: &str) {
    let mut selection = Selection::new();
    for (option, value) in selected_options {
        match option {
            "--instrument" => selection.instrument = value.to_string(),
            "--tuning" => selection.tuning = value.to_string(),
            "--tuning-note" => selection.tuning_note = value.to_string(),
            "--highlight-type" => selection.highlight_type = value.to_string(),
            "--highlight" => selection.highlight = value.to_string(),
            "--highlight-note" => selection.highlight_note = value.to_string(),
            _ => return,
        }
    }
    let data_file = std::fs::File::open(data_file_name);
    if data_file.is_err() {
        eprintln!("'{data_file_name}' not found.");
        return;
    }
    let data_file_lines = std::io::BufReader::new(data_file.unwrap())
        .lines()
        .flatten();

    let mut data: [Option<DataLine>; 4] = [None, None, None, None];
    // 0: Notes, 1: instrument, 2: tuning_formula,
    // 3: highlight_formula

    for line in data_file_lines {
        match verify_data_line(&line) {
            Ok(fields) => match fields {
                DataLine::Notes(_) => {
                    if data.get(0).unwrap().is_none() {
                        data[0] = Some(fields);
                    }
                }
                DataLine::Instrument(ref instrument) => {
                    if data.get(1).unwrap().is_none() {
                        if selection.instrument.eq(instrument) {
                            data[1] = Some(fields);
                        }
                    }
                }
                DataLine::Tuning {
                    ref instrument,
                    ref name,
                    ..
                } => {
                    if data.get(2).unwrap().is_none() {
                        if selection.instrument.eq(instrument) && selection.tuning.eq(name) {
                            data[2] = Some(fields);
                        }
                    }
                }
                DataLine::Highlight {
                    ref h_type,
                    ref name,
                    ..
                } => {
                    if data.get(3).unwrap().is_none() {
                        if selection.highlight_type.eq(h_type) && selection.highlight.eq(name) {
                            data[3] = Some(fields);
                        }
                    }
                }
            },
            Err(err) => {
                eprintln!("{err} - {line}");
                break;
            }
        }
    }
    if let Some(wrong_arg) = data.iter().position(|x| x.is_none()) {
        eprintln!(
            "The given arguments were not found in '{data_file_name}' - {}",
            match wrong_arg {
                1..=3 => format!(
                    "{}, {}, {}, {}",
                    selection.instrument,
                    selection.tuning,
                    selection.highlight_type,
                    selection.highlight
                ),
                _ => "Notes (data file is corrupt.)".to_owned(),
            }
        );
        return;
    }

    if let Some(DataLine::Notes(notes)) = data.get(0).unwrap() {
        if !notes.contains(&selection.tuning_note) {
            eprintln!("Invalid tuning note - {}", selection.tuning_note);
            return;
        }
        if !notes.contains(&selection.highlight_note) {
            eprintln!("Invalid highlight note - {}", selection.highlight_note);
            return;
        }
    }

    print_fretboard(selection, data);
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let version_string: String = format!(
        "{} v({})\n{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
    let mut i = 1;
    let mut data_file: &str; // stores data_file only when selection_options are provided
    let mut selection_options: Vec<(&str, &str)> = vec![];

    while i < args.len() {
        let arg = args.get(i).unwrap();
        // -h, --help
        if arg.eq("--help") || arg.eq("-h") {
            println!("{version_string}{USAGE}");
            return ExitCode::SUCCESS;
        }
        // -v, --version
        if arg.eq("--version") || arg.eq("-v") {
            println!("{version_string}");
            return ExitCode::SUCCESS;
        }

        // list <option>
        if arg.eq("list") {
            if let Some(list_option) = args.get(i + 1) {
                data_file = get_data_file_name(args.get(i + 2));
                let selected_list_option = match list_option.as_str() {
                    "instruments" => "instruments",
                    "tunings" => "tunings",
                    "highlights" => "highlights",
                    "datafile" => "datafile",
                    _ => {
                        eprintln!(
			"unknown \"fretsmart list\" option: {list_option}\nSee 'fretsmart --help'."
			);
                        return ExitCode::FAILURE;
                    }
                };
                process_command_list(selected_list_option, data_file);
                return ExitCode::SUCCESS;
            } else {
                eprintln!(
                    "\"fretsmart list\" requires at least 1 argument.\nSee 'fretsmart --help'."
                );
                return ExitCode::FAILURE;
            }
        } // list <option>

        // update
        if arg.eq("update") {
            data_file = get_data_file_name(args.get(i + 1));
            process_command_update(data_file);
            break;
        }

        // --selection_options
        if arg.starts_with("--") {
            if SELECTION_OPTION_VALUES.contains(&(arg.as_str())) {
                if let Some(option_value) = args.get(i + 1) {
                    selection_options.push((arg.as_str(), option_value.as_str()));
                } else {
                    eprintln!("ignoring \"{arg}\".");
                }
            }
            i = i + 2;
            continue;
        } // --selection_options

        break; // break when no more selection_options to parse
    }
    // final parsed argument if given
    data_file = get_data_file_name(args.get(i));

    process_selection_options(selection_options, data_file);
    ExitCode::SUCCESS
}
