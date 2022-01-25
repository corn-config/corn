use colored::*;
use pest::error::{Error, ErrorVariant, LineColLocation};
use std::path::Path;

use crate::Rule;

pub const ERR_PARSING: i32 = 1;
pub const ERR_VARIABLE: i32 = 2;
pub const ERR_FILE_READ: i32 = 3;

pub fn print_err(message: String, context: Option<String>) {
    if let Some(context) = context {
        eprintln!("{} {}:", "An error occurred".red(), context.red());
    } else {
        eprintln!("{}", "An error occurred:".red());
    }

    eprintln!("\t{}", message.red().bold());
}

pub fn print_parser_err(error: Error<Rule>, file: String, path: &Path) {
    let message = match error.variant {
        ErrorVariant::ParsingError {
            positives,
            negatives: _negatives,
        } => {
            format!(
                "Error found while parsing file.\nExpected one of:\n\t{}",
                positives
                    .iter()
                    .map(|rule| rule.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
        ErrorVariant::CustomError { message } => message,
    }
    .red()
    .bold();

    let pos: ((usize, usize), (usize, usize)) = match error.line_col {
        LineColLocation::Pos((row, col)) => ((row, col), (row, col)),
        LineColLocation::Span((row1, col1), (row2, col2)) => ((row1, col1), (row2, col2)),
    };

    let line = file.lines().nth(pos.0 .0 - 1).unwrap();

    let underline: String = (0..line.len())
        .map(|i| {
            if i >= pos.0 .1 - 1 && i < pos.1 .1 {
                '^'
            } else {
                ' '
            }
        })
        .collect();

    let bar = "  | ".blue();

    eprintln!("--> {}:{}:{}", path.display(), pos.0 .0, pos.0 .1);
    eprintln!("{}", bar);
    eprintln!("{}{}", bar, line.red());
    eprintln!("{}{}", bar, underline.red());
    eprintln!("{}", bar);
    eprintln!("{}", message);
}
