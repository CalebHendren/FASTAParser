use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use FASTAParser::{
    converter::{from_csv, from_json, from_tsv, from_xml},
    gc::run_gc,
    writer::{write_csv, write_fasta, write_json, write_tsv, write_xml},
    models::Record,
};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Input file path (any supported format)
    input: PathBuf,

    /// Optional output file path (stdout if omitted)
    output: Option<PathBuf>,

    /// Compute and print GC content only
    #[arg(long)]
    gc: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.gc {
        return run_gc(&args.input);
    }

    // Detect input format
    let input_ext = args
        .input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let records: Vec<Record> = match input_ext.as_str() {
        "json" => from_json(args.input.to_str().unwrap()).expect("Invalid JSON"),
        "csv"  => from_csv(args.input.to_str().unwrap()).expect("Invalid CSV"),
        "tsv"  => from_tsv(args.input.to_str().unwrap()).expect("Invalid TSV"),
        "xml"  => from_xml(args.input.to_str().unwrap()).expect("Invalid XML"),
        "fasta" | "fa" | "fna" => {
            FASTAParser::parser::parse_fasta(&args.input)?
        }
        _ => {
            eprintln!("Unsupported input format: {}", input_ext);
            std::process::exit(1);
        }
    };

    // Detect output format
    let output_ext = args
        .output
        .as_ref()
        .and_then(|p| p.extension())
        .and_then(|s| s.to_str())
        .unwrap_or("csv")
        .to_lowercase();

    let mut writer: Box<dyn Write> = match &args.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };

    match output_ext.as_str() {
        "json" => write_json(&mut writer, &records).expect("Failed to write JSON"),
        "csv"  => write_csv(&mut writer, &records).expect("Failed to write CSV"),
        "tsv"  => write_tsv(&mut writer, &records).expect("Failed to write TSV"),
        "xml"  => write_xml(&mut writer, &records).expect("Failed to write XML"),
        "fasta" | "fa" | "fna" => write_fasta(&mut writer, &records)?,
        _ => {
            eprintln!("Unsupported output format: {}", output_ext);
            std::process::exit(1);
        }
    }

    Ok(())
}