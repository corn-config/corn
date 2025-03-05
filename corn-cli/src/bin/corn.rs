use corn::{parse, Value};
use corn_cli::error::{print_err, Error, ExitCode};
use std::io::Read;
use std::process::exit;
use std::{fs, io};

use crate::Error::Corn;
use clap::{Parser, ValueEnum};
use colored::Colorize;

#[derive(ValueEnum, Clone, Copy, Debug)]
enum OutputType {
    Json,
    Yaml,
    Toml,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the input corn file. If not set, reads from stdin instead.
    input: Option<String>,

    /// The file format to output
    #[clap(long = "type", short = 't', value_enum)]
    output_type: Option<OutputType>,
}

/// Reads input from file if given a path, otherwise stdin
fn get_input(input: Option<&String>) -> io::Result<String> {
    if let Some(input) = input {
        fs::read_to_string(input)
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

fn main() {
    let args = Args::parse();

    let unparsed_file = get_input(args.input.as_ref());

    match unparsed_file {
        Ok(unparsed_file) => {
            let output_type = get_output_type(args.output_type);

            match parse(&unparsed_file) {
                Ok(config) => match serialize(&config, output_type) {
                    Ok(serialized) => println!("{serialized}"),
                    Err(err) => handle_err(&err),
                },
                Err(err) => handle_err(&Corn(err)),
            };
        }
        Err(err) => {
            print_err(
                &err.to_string(),
                Some(format!(
                    "while attempting to read `{}`",
                    args.input.unwrap_or_else(|| "stdin".to_string()).bold()
                )),
            );

            let error = Error::ReadingFile(err);
            exit(error.get_exit_code());
        }
    }
}

/// Gets the file type to use for the output.
/// If the type arg is supplied, this is used.
/// Otherwise, the output type falls back to JSON as the default.
fn get_output_type(arg: Option<OutputType>) -> OutputType {
    if let Some(output_type) = arg {
        return output_type;
    }

    OutputType::Json
}

fn serialize(config: &Value, output_type: OutputType) -> Result<String, Error> {
    match output_type {
        OutputType::Json => serde_json::to_string_pretty(&config).map_err(Error::from),
        OutputType::Yaml => serde_norway::to_string(&config).map_err(Error::from),
        OutputType::Toml => toml_edit::ser::to_string_pretty(&config).map_err(Error::from),
    }
}

fn handle_err(error: &Error) {
    let code = error.get_exit_code();
    let code_formatted = format!("[E{code:0>4}]").red().bold();

    eprintln!("{code_formatted} {error}");

    exit(code);
}
