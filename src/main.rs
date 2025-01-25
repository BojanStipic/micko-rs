use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::{
    error::{Simple, SimpleReason},
    Parser, Stream,
};
use clap::Parser as ClapParser;

use crate::ast::Program;

mod ast;
mod lexer;
mod parser;

#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// miniC source file
    input: String,
}

fn main() {
    let Cli { input } = Cli::parse();
    let src = std::fs::read_to_string(input).unwrap();

    match lexer::lexer().parse(src.as_str()) {
        Ok(tokens) => {
            let len = src.chars().count();
            let token_stream = Stream::from_iter(len..len + 1, tokens.into_iter());

            match Program::parser().parse(token_stream) {
                Ok(ast) => println!("{ast:#?}"),
                Err(errs) => report_errors(
                    &src,
                    errs.into_iter()
                        .map(|err| err.map(|token| token.to_string()))
                        .collect(),
                ),
            }
        }

        Err(errs) => report_errors(
            &src,
            errs.into_iter()
                .map(|err| err.map(|char| char.to_string()))
                .collect(),
        ),
    }
}

fn report_errors(src: &str, errs: Vec<Simple<String>>) {
    for err in errs {
        let report = Report::build(ReportKind::Error, err.span());

        let report = match err.reason() {
            SimpleReason::Unclosed { span, delimiter } => report
                .with_message(format!(
                    "Unclosed delimiter {}",
                    delimiter.fg(Color::Yellow)
                ))
                .with_label(
                    Label::new(span.clone())
                        .with_message(format!(
                            "Unclosed delimiter {}",
                            delimiter.fg(Color::Yellow)
                        ))
                        .with_color(Color::Yellow),
                )
                .with_label(
                    Label::new(err.span())
                        .with_message(format!(
                            "Must be closed before this {}",
                            err.found()
                                .unwrap_or(&"end of file".to_string())
                                .fg(Color::Red)
                        ))
                        .with_color(Color::Red),
                ),
            SimpleReason::Unexpected => report
                .with_message(format!(
                    "{}, expected {}",
                    if err.found().is_some() {
                        "Unexpected token in input"
                    } else {
                        "Unexpected end of input"
                    },
                    if err.expected().len() == 0 {
                        "something else".to_string()
                    } else {
                        err.expected()
                            .map(|expected| match expected {
                                Some(expected) => expected.to_string(),
                                None => "end of input".to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                ))
                .with_label(
                    Label::new(err.span())
                        .with_message(format!(
                            "Unexpected token {}",
                            err.found()
                                .unwrap_or(&"end of file".to_string())
                                .fg(Color::Red)
                        ))
                        .with_color(Color::Red),
                ),
            SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                Label::new(err.span())
                    .with_message(format!("{}", msg.fg(Color::Red)))
                    .with_color(Color::Red),
            ),
        };

        report.finish().print(Source::from(&src)).unwrap();
    }
}
