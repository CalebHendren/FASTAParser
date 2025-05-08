use std::{env, path::Path};
use crate::parser;

pub fn run_gc<P: AsRef<Path>>(input: P) -> std::io::Result<()> {
    let records = parser::parse_fasta(input.as_ref())?;
    for rec in records {
        let seq = &rec.seq;
        let gc_count = seq.chars().filter(|c| matches!(c, 'G'|'C'|'g'|'c')).count();
        let gc_content = (gc_count as f64 / seq.len() as f64) * 100.0;

        println!(">{} - GC Content: {:.2}%", rec.id, gc_content);
    }
    Ok(())
}