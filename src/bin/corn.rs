use cornfig::error::{print_err, print_parser_err, ERR_FILE_READ, ERR_PARSING};
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
    input: String,

    #[clap(long = "type", short = 't', arg_enum)]
    output_type: Option<OutputType>,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.input);

    let unparsed_file = read_to_string(path);

    match unparsed_file {
        Ok(unparsed_file) => {
            let output_type = get_output_type(args.output_type, path);

            match parse(&unparsed_file) {
                Ok(config) => {
                    let serialized = match output_type {
                        OutputType::Json => serde_json::to_string_pretty(&config.value).unwrap(),
                        OutputType::Yaml => serde_yaml::to_string(&config.value).unwrap(),
                    };

                    println!("{}", serialized);
                }
                Err(error) => {
                    print_parser_err(error, unparsed_file, path);
                    exit(ERR_PARSING);
                }
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
            exit(ERR_FILE_READ);
        }
    }
}

fn get_output_type(arg: Option<OutputType>, path: &Path) -> OutputType {
    if let Some(output_type) = arg {
        return output_type;
    }

    if let Some(extension) = path.extension() {
        let extension = extension
            .to_str()
            .expect("Input path contained unsupported characters");

        return match extension {
            "json" => OutputType::Json,
            "yaml" => OutputType::Yaml,
            _ => OutputType::Json,
        };
    }

    OutputType::Json
}
