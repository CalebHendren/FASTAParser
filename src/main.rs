use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use FASTAParser::{
    converter::{from_csv, from_json, from_tsv, from_xml},
    gc::run_gc,
    parser,
    stats::run_stats,
    transcription::run_transcription,
    writer::{write_csv, write_fasta, write_json, write_tsv, write_xml},
    models::Record,
};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    input: PathBuf,
    /// Optional output file path
    output: Option<PathBuf>,
    /// Compute and print GC content only
    #[arg(long)]
    gc: bool,
    /// Generate length & GC% stats & plots
    #[arg(long)]
    stats: bool,
    /// Transcribe DNA to RNA (T->U)
    #[arg(long)]
    transcribe: bool,
    /// Use mRNA mode (remove introns) during transcription
    #[arg(long)]
    mrna: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // stats
    if args.stats {
        run_stats(&args.input)?;
        return Ok(());
    }

    // GC
    if args.gc {
        run_gc(&args.input)?;
        return Ok(());
    }

    // Transcription
    if args.transcribe {
        run_transcription(&args.input, args.mrna)?;
        return Ok(());
    }

    // Convert
    let input_ext = args
        .input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let records: Vec<Record> = match input_ext.as_str() {
        "json" => from_json(args.input.to_str().unwrap()).map_err(|e| e.to_string())?,
        "csv" => from_csv(args.input.to_str().unwrap()).map_err(|e| e.to_string())?,
        "tsv" => from_tsv(args.input.to_str().unwrap()).map_err(|e| e.to_string())?,
        "xml" => from_xml(args.input.to_str().unwrap()).map_err(|e| e.to_string())?,
        "fasta" | "fa" | "fna" => parser::parse_fasta(&args.input)?,
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
        "json" => write_json(&mut writer, &records)?,
        "csv" => write_csv(&mut writer, &records)?,
        "tsv" => write_tsv(&mut writer, &records)?,
        "xml" => write_xml(&mut writer, &records)?,
        "fasta" | "fa" | "fna" => write_fasta(&mut writer, &records)?,
        _ => {
            eprintln!("Unsupported output format: {}", output_ext);
            std::process::exit(1);
        }
    }

    Ok(())
}