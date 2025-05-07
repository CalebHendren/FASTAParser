use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use FASTAParser::run;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Input FASTA file path
    input: PathBuf,

    /// Optional output file path (stdout if omitted)
    output: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let format = match &args.output {
        Some(path) => match path.extension().and_then(|s| s.to_str()) {
            Some("json") => "json",
            Some("csv") => "csv",
            Some("tsv")  => "tsv",
            Some("xml")  => "xml",
            _ => "csv", // default format
        },
        None => "csv", // default format
    };

    let writer: Box<dyn Write> = match &args.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(std::io::stdout()),
    };

    run(&args.input, format, writer)
}
