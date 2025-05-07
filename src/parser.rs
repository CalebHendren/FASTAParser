use crate::models::Record;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse_fasta(path: &std::path::Path) -> io::Result<Vec<Record>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();
    let mut current_id = String::new();
    let mut current_seq = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            if !current_id.is_empty() {
                records.push(Record {
                    id: current_id.clone(),
                    seq: current_seq.clone(),
                });
                current_seq.clear();
            }
            current_id = line[1..].to_string();
        } else {
            current_seq.push_str(line.trim());
        }
    }
    if !current_id.is_empty() {
        records.push(Record { id: current_id, seq: current_seq });
    }
    Ok(records)
}
