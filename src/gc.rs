use std::{env, path::Path};
use crate::parser;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

/// Wrap G/C characters in ANSI red
fn color_gc(seq: &str) -> String {
    seq.chars()
        .map(|c| match c {
            'G' | 'C' | 'g' | 'c' => format!("{}{}{}", RED, c, RESET),
            other => other.to_string(),
        })
        .collect()
}

/// Parse FASTA, compute GC content, and print with G/C in red
pub fn run_gc<P: AsRef<Path>>(input: P) -> std::io::Result<()> {
    let records = parser::parse_fasta(input.as_ref())?;
    for rec in records {
        let seq = &rec.seq;
        let gc_count = seq.chars().filter(|c| matches!(c, 'G'|'C'|'g'|'c')).count();
        let gc_content = (gc_count as f64 / seq.len() as f64) * 100.0;

        println!(">{}", rec.id);
        println!("GC Content: {:.2}%", gc_content);
        println!("{}", color_gc(seq));
    }
    Ok(())
}