use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use FASTAParser::run;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Input FASTA file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output format: fasta (default), json, or csv
    #[arg(short, long, default_value = "fasta")]
    format: String,

    /// Optional output file path (stdout if omitted)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let writer: Box<dyn Write> = match &args.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(std::io::stdout()),
    };

    run(&args.input, &args.format, writer)
}
