pub mod models;
pub mod parser;
pub mod writer;
use std::path::Path;
use std::io::{self, Write};

pub fn run<P: AsRef<Path>, W: Write>(
    input: P,
    format: &str,
    mut out: W,
) -> io::Result<()> {
    let recs = parser::parse_fasta(input.as_ref())?;
    match format {
        "json" => writer::write_json(&mut out, &recs)?,
        "csv"  => writer::write_csv(&mut out, &recs)?,
        _      => writer::write_fasta(&mut out, &recs)?,
    }
    Ok(())
}
