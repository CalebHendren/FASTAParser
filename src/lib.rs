pub mod models;
pub mod parser;
pub mod writer;
pub mod converter;
pub mod gc;
use std::path::Path;
use std::io::{self, Write};

pub fn run<P: AsRef<Path>, W: Write>(
    input: P,
    format: &str,
    mut out: W,
) -> io::Result<()> {
    // parse_fasta returns io::Result<Vec<Record>>
    let records = parser::parse_fasta(input.as_ref())?;

    match format {
        "json" => writer::write_json(&mut out, &records)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,

        "csv" => writer::write_csv(&mut out, &records)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,

        "tsv" => writer::write_tsv(&mut out, &records)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,

        "xml" => writer::write_xml(&mut out, &records)?,

        // treat both `.fasta` and anything else as FASTA
        "fasta" | _ => writer::write_fasta(&mut out, &records)?,
    }

    Ok(())
}
