use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use FASTAParser::{run as parse_run, gc::run_gc};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Input FASTA file path
    input: PathBuf,

    /// Optional output file path (stdout if omitted)
    output: Option<PathBuf>,

    /// Compute and print GC content (with G/C in red)
    #[arg(long)]
    gc: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.gc {
        // GC mode ignores output file
        return run_gc(&args.input);
    }

    // parse mode
    let format = match &args.output {
        Some(path) => match path.extension().and_then(|s| s.to_str()) {
            Some("json") => "json",
            Some("csv")  => "csv",
            Some("tsv")  => "tsv",
            Some("xml")  => "xml",
            _            => "fasta",
        },
        None => "csv",
    };

    let writer: Box<dyn Write> = match &args.output {
        Some(path) => Box::new(File::create(path)?),
        None       => Box::new(io::stdout()),
    };

    parse_run(&args.input, format, writer)
}
