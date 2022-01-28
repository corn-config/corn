use cornfig::error::{format_parser_err, print_err, Error, ExitCode, FileReadError};
use cornfig::parse;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;

use clap::{ArgEnum, Parser};
use colored::*;

#[derive(ArgEnum, Clone, Debug)]
enum OutputType {
    Json,
    Yaml,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the input corn file
    input: String,

    /// The file format to output
    #[clap(long = "type", short = 't', arg_enum)]
    output_type: Option<OutputType>,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.input);

    let unparsed_file = read_to_string(path);

    match unparsed_file {
        Ok(unparsed_file) => {
            let output_type = get_output_type(args.output_type);

            match parse(&unparsed_file) {
                Ok(config) => match serialize(config, output_type) {
                    Ok(serialized) => println!("{}", serialized),
                    Err(err) => handle_err(err, unparsed_file, path),
                },
                Err(err) => handle_err(err, unparsed_file, path),
            };
        }
        Err(err) => {
            print_err(
                err.to_string(),
                Some(format!(
                    "while attempting to read `{}`",
                    path.display().to_string().bold()
                )),
            );
            exit(FileReadError::EXIT_CODE);
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

fn handle_err(error: Error, unparsed_file: String, path: &Path) {
    let code = error.get_exit_code();
    let code_formatted = format!("[E{:0>4}]", code).red();

    eprintln!("{} {}", code_formatted, error.to_string().bright_red());

    if let Error::ParserError(err) = error {
        eprintln!("{}", format_parser_err(err, unparsed_file, path));
    };

    exit(code);
}
