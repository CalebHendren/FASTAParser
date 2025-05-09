use std::path::Path;
use std::io;
use crate::{
    converter::{from_csv, from_json, from_tsv, from_xml},
    parser,
    models::Record,
};

/// Compute GC% for a single sequence (0.0–100.0)
pub fn gc_content(seq: &str) -> f64 {
    let gc = seq
        .chars()
        .filter(|c| matches!(c, 'G' | 'C' | 'g' | 'c'))
        .count();
    (gc as f64 / seq.len() as f64) * 100.0
}

/// Compute and print GC content from any supported file format
pub fn run_gc<P: AsRef<Path>>(input: P) -> io::Result<()> {
    let path = input.as_ref();
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let result: Result<Vec<Record>, io::Error> = match ext.as_str() {
        "json" => from_json(path.to_str().unwrap())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string())),
        "csv" => from_csv(path.to_str().unwrap())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string())),
        "tsv" => from_tsv(path.to_str().unwrap())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string())),
        "xml" => from_xml(path.to_str().unwrap()),
        "fasta" | "fa" | "fna" => parser::parse_fasta(path),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported format: {}", ext),
            ));
        }
    };

    let records = result?;

    for rec in records {
        let gc = gc_content(&rec.seq);
        println!(">{} - GC Content: {:.2}%", rec.id, gc);
    }

    Ok(())
}