use crate::models::Record;
use std::fs::File;
use std::io::{self, BufReader};
use csv::ReaderBuilder;

/// Parse JSON to Records
pub fn from_json(path: &str) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let records = serde_json::from_reader(reader)?;
    Ok(records)
}

/// Parse CSV to Records
pub fn from_csv(path: &str) -> csv::Result<Vec<Record>> {
    let mut rdr = ReaderBuilder::new().delimiter(b',').from_path(path)?;
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        records.push(Record {
            id: record.get(0).unwrap_or_default().to_string(),
            seq: record.get(1).unwrap_or_default().to_string(),
        });
    }
    Ok(records)
}

/// Parse TSV to Records
pub fn from_tsv(path: &str) -> csv::Result<Vec<Record>> {
    let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_path(path)?;
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        records.push(Record {
            id: record.get(0).unwrap_or_default().to_string(),
            seq: record.get(1).unwrap_or_default().to_string(),
        });
    }
    Ok(records)
}

/// Parse XML to Records (very basic)
pub fn from_xml(path: &str) -> io::Result<Vec<Record>> {
    let content = std::fs::read_to_string(path)?;
    let mut records = Vec::new();

    for rec in content.split("<record>") {
        if rec.contains("<id>") && rec.contains("<sequence>") {
            let id = rec
                .split("<id>").nth(1).unwrap_or("")
                .split("</id>").next().unwrap_or("")
                .trim()
                .to_string();

            let seq = rec
                .split("<sequence>").nth(1).unwrap_or("")
                .split("</sequence>").next().unwrap_or("")
                .trim()
                .to_string();

            if !id.is_empty() && !seq.is_empty() {
                records.push(Record { id, seq });
            }
        }
    }

    Ok(records)
}
